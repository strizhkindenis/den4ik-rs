#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

use std::{
    collections::HashMap,
    fs,
    io::{self, Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    str::FromStr,
    thread,
};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const MAX_HEADERS_SIZE: usize = 1024;
const MAX_CONTENT_SIZE: usize = 1024;

const HEADER_CONTENT_LENGTH: &str = "CONTENT_LENGTH";
const HEADER_SCGI: &str = "SCGI";
const HEADER_METHOD: &str = "REQUEST_METHOD";
const HEADER_URI: &str = "REQUEST_URI";

#[derive(Debug, Clone, Copy)]
pub enum Method {
    GET,
    POST,
    DELETE,
}

impl FromStr for Method {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "DELETE" => Ok(Method::DELETE),
            value => Err(value.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum SCGIError {
    HeadersIo(io::Error),
    HeadersInvalidLength,
    HeadersTooLarge(usize),
    HeadersNoClosingComma,
    MissingHeader(&'static str),
    InvalidHeader(&'static str, String),
    InvalidMethod(String),
    ContentIo(io::Error),
    ContentInvalidLength,
    ContentTooLarge(usize, usize),
    InvalidConnection,
}

pub struct Response {
    status: u16,
    content_type: String,
    content: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: 200,
            content_type: String::from("text/plain; charset=utf-8"),
            content: Vec::new(),
        }
    }
}

impl Response {
    #[must_use] 
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    #[must_use] 
    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = content_type;
        self
    }

    #[must_use] 
    pub fn with_content(mut self, content: Vec<u8>) -> Self {
        self.content = content;
        self
    }
}

pub struct Ctx<S> {
    uri: String,
    method: Method,
    headers: HashMap<String, String>,
    content: Vec<u8>,
    state: S,
}

struct ConCtx<S, F> {
    state: S,
    handler: F,
    stream: UnixStream,
    max_content_len: usize,
}

fn get_header<'l>(
    headers: &'l HashMap<String, String>,
    header: &'static str,
) -> Result<&'l str, SCGIError> {
    headers
        .get(header)
        .ok_or(SCGIError::MissingHeader(header))
        .map(std::string::String::as_str)
}

fn get_header_as<T>(headers: &HashMap<String, String>, header: &'static str) -> Result<T, SCGIError>
where
    T: FromStr,
{
    let value = get_header(headers, header)?;
    value
        .parse::<T>()
        .map_err(|_| SCGIError::InvalidHeader(header, value.to_string()))
}

impl<S, F> ConCtx<S, F> {
    fn new(stream: UnixStream, state: S, handler: F, max_content_len: Option<usize>) -> Self {
        let max_content_len = max_content_len
            .unwrap_or(MAX_CONTENT_SIZE)
            .min(MAX_CONTENT_SIZE);
        Self {
            state,
            handler,
            stream,
            max_content_len,
        }
    }

    fn parse_headers_len(&mut self) -> Result<usize, SCGIError> {
        let mut c_buf = [0u8; 1];
        let mut n = 0;
        loop {
            self.stream
                .read_exact(&mut c_buf)
                .map_err(SCGIError::HeadersIo)?;
            let c = c_buf[0];
            if c == b':' {
                break;
            }
            if !c.is_ascii_digit() {
                return Err(SCGIError::HeadersInvalidLength);
            }
            n = n * 10 + usize::from(c - b'0');
            if n > MAX_HEADERS_SIZE {
                break;
            }
        }
        Ok(n)
    }

    fn parse_headers(&mut self, headers_len: usize) -> Result<HashMap<String, String>, SCGIError> {
        if headers_len > MAX_HEADERS_SIZE {
            return Err(SCGIError::HeadersTooLarge(headers_len));
        }
        let mut headers_buf = vec![0u8; headers_len];
        self.stream
            .read_exact(&mut headers_buf)
            .map_err(SCGIError::HeadersIo)?;
        let mut c_buf = [0u8; 1];
        self.stream
            .read_exact(&mut c_buf)
            .map_err(SCGIError::HeadersIo)?;
        if c_buf[0] != b',' {
            return Err(SCGIError::HeadersNoClosingComma);
        }
        let mut headers_iter = headers_buf
            .split(|b| b'\0'.eq(b))
            .map(String::from_utf8_lossy)
            .map(String::from);
        let mut headers = HashMap::new();
        while let Some(key) = headers_iter.next() {
            let value = headers_iter.next().unwrap_or_default();
            headers.insert(key, value);
        }
        Ok(headers)
    }

    fn parse_content(&mut self, content_len: usize) -> Result<Vec<u8>, SCGIError> {
        if content_len > self.max_content_len {
            return Err(SCGIError::ContentTooLarge(
                content_len,
                self.max_content_len,
            ));
        }
        let mut content_buf = vec![0u8; content_len];
        self.stream
            .read_exact(&mut content_buf)
            .map_err(SCGIError::ContentIo)?;
        Ok(content_buf)
    }

    fn parse_scgi(mut self) -> Result<(), SCGIError>
    where
        S: Clone,
        F: Fn(Ctx<S>) -> Response,
    {
        let headers_len = self.parse_headers_len()?;
        let headers = self.parse_headers(headers_len)?;
        let scgi_version = get_header(&headers, HEADER_SCGI)?;
        if scgi_version != "1" {
            return Err(SCGIError::InvalidHeader(
                HEADER_SCGI,
                scgi_version.to_string(),
            ));
        }
        let content_len = get_header_as(&headers, HEADER_CONTENT_LENGTH)?;
        let method = get_header(&headers, HEADER_METHOD)?
            .parse::<Method>()
            .map_err(SCGIError::InvalidMethod)?;
        let uri = get_header(&headers, HEADER_URI)?.to_string();
        let content = self.parse_content(content_len)?;
        let ctx = Ctx {
            uri,
            method,
            headers,
            content,
            state: self.state.clone(),
        };
        let response = (self.handler)(ctx);
        self.write_response(response)
    }

    fn write_response(&mut self, response: Response) -> Result<(), SCGIError> {
        let status_text = match response.status {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        };
        let header = format!(
            "Status: {} {}\r\nContent-Type: {}\r\n\r\n",
            response.status, status_text, response.content_type
        );
        self.stream
            .write_all(header.as_bytes())
            .map_err(SCGIError::HeadersIo)?;
        self.stream
            .write_all(&response.content)
            .map_err(SCGIError::ContentIo)?;
        Ok(())
    }
}

fn remove_file(path: &Path) {
    let exists = path.try_exists().unwrap_or_else(|_| {
        panic!(
            "{CRATE_NAME}: couldn't verify existence of file: {}",
            path.display()
        )
    });
    if !exists {
        return;
    }
    fs::remove_file(path)
        .unwrap_or_else(|_| panic!("{CRATE_NAME}: couldn't delete file: {}", path.display()));
}

pub struct SCGI<S, F> {
    state: S,
    handler: F,
}

impl<S, F> SCGI<S, F> {
    pub fn new(state: S, handler: F) -> Self {
        Self { state, handler }
    }

    pub fn run<P: AsRef<Path>>(self, path: P)
    where
        S: Clone + Send + 'static,
        F: Fn(Ctx<S>) -> Response + Sync + Send + 'static,
    {
        let path = path.as_ref();
        remove_file(path);
        let listener = UnixListener::bind(path).unwrap_or_else(|_| {
            panic!(
                "{CRATE_NAME}: failed to bind to the socket: {}",
                path.display()
            )
        });
        let _ = thread::scope(|s| {
            for stream in listener.incoming() {
                let Ok(stream) = stream else {
                    return Err(SCGIError::InvalidConnection);
                };
                let state = self.state.clone();
                let con_ctx = ConCtx::new(stream, state, &self.handler, None);
                s.spawn(move || {
                    let _ = con_ctx.parse_scgi();
                });
            }
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
