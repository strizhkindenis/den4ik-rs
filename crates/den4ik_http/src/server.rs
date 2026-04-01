use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::frame::{FRAME_HEADER_SIZE, Frame, FrameHeader, RawFrame};

const PREFACE: &[u8; 24] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

pub struct Http2Server {}

impl Http2Server {
    pub fn start(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            handle_client(stream.unwrap());
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    let mut frame_header_buf = [0; FRAME_HEADER_SIZE];
    let mut buf: Vec<u8> = vec![0; 1024];
    stream.read_exact(&mut buf[..PREFACE.len()]).unwrap();
    assert_eq!(&buf[..PREFACE.len()], PREFACE);
    println!("peer: {peer} | PREFACE");
    loop {
        let frame = read_frame(&mut stream, &mut frame_header_buf, &mut buf);
        println!("peer: {peer} | {frame:#?}");
    }
}

fn read_frame(
    stream: &mut TcpStream,
    header_buf: &mut [u8; FRAME_HEADER_SIZE],
    data_buf: &mut [u8],
) -> Frame {
    stream.read_exact(header_buf).unwrap();
    let header: FrameHeader = (header_buf as &[u8; 9]).try_into().unwrap();
    let data = &mut data_buf[..header.length.try_into().unwrap()];
    stream.read_exact(data).unwrap();
    let raw_frame = RawFrame::new(header, data);
    raw_frame.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::Http2Server;

    // #[test]
    // fn run() {
    //     let server = Http2Server {};
    //     server.start()
    // }
}
