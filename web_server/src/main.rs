//use std::io::prelude::*;
//use hello::ThreadPool;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::{Duration};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "/home/zby/rustworkspace/web_server/src/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else   {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "/home/zby/rustworkspace/web_server/src/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();



}






























































//    if buffer.starts_with(get) {
//
//        let mut file = File::open("/home/zby/rustworkspace/web_server/src/hello.html").unwrap();
//
//        let mut contents = String::new();
//
//        file.read_to_string(&mut contents).unwrap();
//
//        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", contents);
//
//        stream.write(response.as_bytes()).unwrap();
//
//        stream.flush().unwrap();
//    } else {
//        let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
//        let mut file = File::open("/home/zby/rustworkspace/web_server/src/404.html").unwrap();
//        let mut contents = String::new();
//
//        file.read_to_string(&mut contents).unwrap();
//
//        let response = format!("{}{}", header, contents);
//
//        stream.write(response.as_bytes()).unwrap();
//        stream.flush().unwrap();
//    };


