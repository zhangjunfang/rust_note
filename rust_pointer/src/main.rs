use std::net:: TcpStream;
use std::string::String;
use std::io::Write;
use std::io::Read;
//client
fn main() {
    let mut stream = TcpStream::connect("0.0.0.0:8080").unwrap();
    let mut s:Vec<u8> = Vec::new();
    const CHUNK_SIZE: usize = 2048;

    let mut buf = [0; CHUNK_SIZE];
    while let Ok(n) = stream.read(&mut buf) {

        s.extend_from_slice(&buf[0..n]);

        if n != CHUNK_SIZE {

            break ;

        }
    }
    let sparkle_heart= String::from_utf8(s);

    match sparkle_heart {
        Ok(ref t)=>{

            println!("--------{:?}",*t);
        },
        Err(e)=>{println!("------===={:?}",e);},
    }

   // println!("......={:?}",s);
    stream.write(b"GET / HTTP/1.0\n\n");  //获取发送网页
}
