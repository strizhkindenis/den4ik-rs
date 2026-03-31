use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::frame::Frame;

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
    let mut mbuf: Vec<u8> = vec![0; 1024];
    let n = stream.read(&mut mbuf).unwrap();
    let buf = &mbuf[..n];
    let (preface, buf) = buf.split_at_checked(PREFACE.len()).unwrap();
    assert_eq!(preface, PREFACE);
    println!("peer: {peer} | PREFACE");
    println!("peer: {peer} | buf: {buf:?}");
    let frame = Frame::try_from(buf).unwrap();
    let (_, buf) = buf.split_at(frame.get_size().try_into().unwrap());
    println!("peer: {peer} | frame: {frame:#?}");
    println!("peer: {peer} | buf: {buf:?}");
    let frame = Frame::try_from(buf).unwrap();
    let (_, buf) = buf.split_at(frame.get_size().try_into().unwrap());
    println!("peer: {peer} | frame: {frame:#?}");
    println!("peer: {peer} | buf: {buf:?}");
    let frame = Frame::try_from(buf).unwrap();
    let (_, buf) = buf.split_at(frame.get_size().try_into().unwrap());
    println!("peer: {peer} | frame: {frame:#?}");
    println!("peer: {peer} | buf: {buf:?}");
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
