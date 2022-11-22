use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

pub fn server_a() {
    let lsn = TcpListener::bind("127.0.0.1:3100").unwrap();
    println!("Running on port 3100 ...");
    for stream in lsn.incoming() {
        handle_stream_a(&mut stream.unwrap(), 3);
    }
}

fn handle_stream_a(stream: &mut TcpStream, wait_time: u64) {
    let mut buffer = [0; 1024];
    let _byte_read = stream.read(&mut buffer).unwrap();
    thread::sleep(Duration::from_secs(wait_time));
    // stream.write(&buffer[..byte_read]).unwrap();
    // stream.write("\n".as_bytes()).unwrap();
}

pub fn server_b() {
    let lsn = TcpListener::bind("127.0.0.1:3200").unwrap();
    println!("Running on port 3200 ...");
    for stream in lsn.incoming() {
        handle_stream_b(&mut stream.unwrap(), 1);
    }
}

fn handle_stream_b(stream: &mut TcpStream, wait_time: u64) {
    let mut buffer = [0; 1024];
    let _byte_read = stream.read(&mut buffer).unwrap();
    thread::sleep(Duration::from_secs(wait_time));
    // stream.write(&buffer[..byte_read]).unwrap();
    // stream.write("\n".as_bytes()).unwrap();
}

pub fn use_server(server: &str, port: u16, content: &str) {
    let mut stream = TcpStream::connect((server, port)).unwrap();
    let _ = stream.write(content.as_bytes()).unwrap();

    let mut reader = BufReader::new(stream);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_until(b'\n', &mut buffer).unwrap();
    println!("recv from sevrer: {}", String::from_utf8(buffer).unwrap());
}

fn main() {
    println!("why_async");
}
