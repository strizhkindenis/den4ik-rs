use std::{
	array,
	io::Read,
	fs,
	path::Path,
	borrow::Cow,
	os::unix::net::{UnixListener, UnixStream},
	collections::HashMap,
};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const MAX_HEADERS_SIZE: usize = 1024;
const MAX_CONTENT_TEXT_SIZE: usize = 1024;
const MAX_CONTENT_FILE_SIZE: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy)]
enum Method {
	GET,
	POST,
	DELETE
}

pub enum Content<'l> {
	FILE(&'l [u8]),
	TEXT(&'l str),
}

impl<'l> Content<'l> {
	fn new_file(data: &'l [u8]) -> Self {
		assert!(data.len() <= MAX_CONTENT_FILE_SIZE);
		Self::FILE(data)
	}
	
	fn new_text(data: &'l str) -> Self {
		assert!(data.len() <= MAX_CONTENT_TEXT_SIZE);	
		Self::TEXT(data)
	}
	
	pub fn get_text(&self) -> Option<&'l str> {
		match self {
			Content::FILE(_) => None,
			Content::TEXT(data) => {
				if data.is_empty() {
					None
				} else {
					Some(data)	
				}
			}
		}
	}
	
	pub fn get_file(&self) -> Option<&'l [u8]> {
		match self {
			Content::TEXT(_) => None,
			Content::FILE(data) => {
				if data.is_empty() {
					None
				} else {
					Some(data)	
				}
			}
		}
	}
}


pub struct Ctx<'l> {
	method: Method,
	uri: &'l str,
	content: Content<'l>,
}

struct ConCtx<S> {
	state: S,
	stream: UnixStream,
	text_buf: [u8; MAX_CONTENT_TEXT_SIZE],
	file_buf: Vec<u8>,
}

impl<S> ConCtx<S> {
	fn new(stream: UnixStream, state: S) -> Self {
		Self {
			state,
			stream,
			text_buf: array::from_fn(|_| Default::default()), 	
			file_buf: Vec::default(),
		}
	}
	
	fn parse_header_len(&mut self) -> usize {
		let mut c_buf = [0u8; 1];	
		let mut n = 0;
		loop {
			self.stream.read_exact(&mut c_buf)
				.expect(&format!("{CRATE_NAME}: couldn't parse headers length"));
			let c = char::try_from(c_buf[0])
				.expect(&format!("{CRATE_NAME}: couldn't parse headers length"));
			if c == ':' {
				break;
			}
			let c = usize::try_from(c)
				.expect(&format!("{CRATE_NAME}: couldn't parse headers length"));
			n += n * 10 + c;
		}
		n
	}
	
	fn parse_headers(&mut self, headers_len: usize) -> HashMap<String, String> {
		if headers_len > MAX_HEADERS_SIZE {
			panic!("{CRATE_NAME}: headers size > MAX_HEADERS_SIZE({MAX_HEADERS_SIZE}");	
		}
		let mut headers_buf = vec![0u8; headers_len];
		self.stream.read_exact(&mut headers_buf)
			.expect(&format!("{CRATE_NAME}: couldn't read headers data"));
		let mut c_buf = [0u8; 1];	
		self.stream.read_exact(&mut c_buf)
			.expect(&format!("{CRATE_NAME}: expected ',' after headers data"));
		if char::try_from(c_buf[0])
			.expect(&format!("{CRATE_NAME}: expected ',' after headers data")) != ',' {
			panic!("{CRATE_NAME}: expected ',' after headers data");	
		}
		let mut headers_iter = headers_buf.split(|b| 0u8.eq(b)).map(String::from_utf8_lossy).map(String::from);
		let mut headers = HashMap::new();
		while let Some(key) = headers_iter.next() {
			let value = headers_iter.next().unwrap_or_else(|| "".to_string());
			headers.insert(key, value);
		}
		headers
	}
}

fn remove_file(path: &Path) {
	let exists = path
		.try_exists()
		.expect(&format!("{CRATE_NAME}: couldn't verify existence of file: {}", path.display()));
	if !exists {
		return;	
	}
	fs::remove_file(path)
		.expect(&format!("{CRATE_NAME}: couldn't delete file: {}", path.display()));
}

pub struct SCGI<S>(S);

impl<S> SCGI<S> {
	pub fn new(state: S) -> Self {
		Self(state)
	}
	
	pub fn run<P: AsRef<Path>>(&self, path: P)
	where S: Clone {
		let path = path.as_ref();
		remove_file(path);
		let listener = UnixListener::bind(path)
			.expect(&format!("{CRATE_NAME}: failed to bind to the socket: {}", path.display()));
		for stream in listener.incoming() {
			let stream = stream
				.expect(&format!("{CRATE_NAME}: failed to open connection: {}", path.display()));
			let state = self.0.clone();
			let con_ctx = ConCtx::new(stream, state);		
		}
	}
}

fn sample_handler<'l, S>(ctx: Ctx<'l>) {
	println!("Method: {:?}", ctx.method);
	println!("URI: {}", ctx.uri);
	if let Some(text) = ctx.content.get_text() {
		println!("Text: {}", text);	
	}
	if let Some(file) = ctx.content.get_file() {
		println!("FILE: {}KB", file.len() / 1024);	
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
