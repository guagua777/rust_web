use std::net::TcpListener;
use std::io::Read;
use std::io::Write;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("running on port 3000");

    // 只获取一次
    // let result = listener.accept().unwrap();

    // Returns an iterator over the connections being received on this listener.
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connected: {:?}", stream);
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();

        // println!("Received: {:?}", String::from_utf8_lossy(&buffer));
    }
}
