use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::str;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    println!("Connected: {:?}", stream);
    println!("Hello, world!");

    stream.write(b"Hello, world!").unwrap();

    // 等待服务器响应
    
    let mut buffer = [0; 13];
    stream.read(&mut buffer).unwrap();

    println!("Received: {:?}", String::from_utf8_lossy(&buffer));
}
