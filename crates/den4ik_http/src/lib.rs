#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo
)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use den4ik_thread_pool::ThreadPool;

pub struct Http2Server {
    listener: TcpListener,
    pool: ThreadPool,
}

impl Http2Server {
    /// Creates a new HTTP/2 server.
    ///
    /// # Errors
    ///
    /// Returns an error if binding to the address fails.
    ///
    /// # Panics
    ///
    /// Panics if `thread_count` is 0.
    pub fn new(bind_addr: &str, thread_count: usize) -> std::io::Result<Self> {
        let listener = TcpListener::bind(bind_addr)?;
        let pool = ThreadPool::new(std::num::NonZeroUsize::new(thread_count).unwrap());
        Ok(Self { listener, pool })
    }

    pub fn run(self) {
        let listener = Arc::new(self.listener);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.pool.submit(move || {
                        if let Err(e) = handle_connection(stream) {
                            eprintln!("Connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {e}");
                }
            }
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // Read the HTTP/2 connection preface
    let mut preface = [0u8; 24];
    stream.read_exact(&mut preface)?;
    if &preface != b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid HTTP/2 preface",
        ));
    }

    // Send empty SETTINGS frame
    let empty_settings = [0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    stream.write_all(&empty_settings)?;

    // Read frames in a loop
    loop {
        let mut header = [0u8; 9];
        if stream.read_exact(&mut header).is_err() {
            break; // Connection closed or error
        }

        let length = ((header[0] as usize) << 16) | ((header[1] as usize) << 8) | (header[2] as usize);
        let frame_type = header[3];
        let stream_id = u32::from_be_bytes([header[5] & 0x7F, header[6], header[7], header[8]]);

        let mut payload = vec![0u8; length];
        stream.read_exact(&mut payload)?;

        // If we receive a HEADERS frame, we respond with our own HEADERS and DATA
        if frame_type == 0x01 { // HEADERS
            // Prepare response HEADERS frame
            // Payload: HPACK encoded headers. 0x88 = :status: 200
            let header_payload = [0x88];
            let header_len = header_payload.len() as u32;

            let mut resp_header_frame = [0u8; 9];
            resp_header_frame[0..3].copy_from_slice(&header_len.to_be_bytes()[1..4]);
            resp_header_frame[3] = 0x01; // HEADERS type
            resp_header_frame[4] = 0x04; // END_HEADERS flag
            resp_header_frame[5..9].copy_from_slice(&stream_id.to_be_bytes());

            stream.write_all(&resp_header_frame)?;
            stream.write_all(&header_payload)?;

            // Prepare response DATA frame
            let data_payload = b"Hello from den4ik HTTP/2.0 Server!";
            let data_len = data_payload.len() as u32;

            let mut resp_data_frame = [0u8; 9];
            resp_data_frame[0..3].copy_from_slice(&data_len.to_be_bytes()[1..4]);
            resp_data_frame[3] = 0x00; // DATA type
            resp_data_frame[4] = 0x01; // END_STREAM flag
            resp_data_frame[5..9].copy_from_slice(&stream_id.to_be_bytes());

            stream.write_all(&resp_data_frame)?;
            stream.write_all(data_payload)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_http2_server_basic_response() {
        let server = Http2Server::new("127.0.0.1:0", 2).unwrap();
        let addr = server.listener.local_addr().unwrap();

        thread::spawn(move || {
            server.run();
        });

        thread::sleep(Duration::from_millis(100));

        let mut client = TcpStream::connect(addr).unwrap();
        client.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

        // Send connection preface
        client.write_all(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n").unwrap();

        // Send empty SETTINGS frame
        let empty_settings = [0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
        client.write_all(&empty_settings).unwrap();

        // Send a mock HEADERS frame (stream ID 1)
        let header_frame = [
            0x00, 0x00, 0x00, // length 0
            0x01,             // type HEADERS
            0x04,             // flag END_HEADERS
            0x00, 0x00, 0x00, 0x01, // stream ID 1
        ];
        client.write_all(&header_frame).unwrap();

        // Read server SETTINGS frame
        let mut server_settings = [0u8; 9];
        client.read_exact(&mut server_settings).unwrap();
        assert_eq!(server_settings[3], 0x04); // Expect SETTINGS type

        // Read server HEADERS frame
        let mut server_headers = [0u8; 9];
        client.read_exact(&mut server_headers).unwrap();
        assert_eq!(server_headers[3], 0x01); // Expect HEADERS type

        // Read HEADERS payload (length is in first 3 bytes)
        let header_len = ((server_headers[0] as usize) << 16) | ((server_headers[1] as usize) << 8) | (server_headers[2] as usize);
        let mut header_payload = vec![0u8; header_len];
        client.read_exact(&mut header_payload).unwrap();
        assert_eq!(header_payload[0], 0x88); // :status 200

        // Read server DATA frame
        let mut server_data = [0u8; 9];
        client.read_exact(&mut server_data).unwrap();
        assert_eq!(server_data[3], 0x00); // Expect DATA type

        // Read DATA payload
        let data_len = ((server_data[0] as usize) << 16) | ((server_data[1] as usize) << 8) | (server_data[2] as usize);
        let mut data_payload = vec![0u8; data_len];
        client.read_exact(&mut data_payload).unwrap();
        assert_eq!(data_payload, b"Hello from den4ik HTTP/2.0 Server!");
    }
}
