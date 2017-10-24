extern crate threading;
extern crate redis;
use redis::Commands;
use std::thread;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use threading::Pool;
use std::time::Duration;
use std::io::Write;
///     let ip = Ipv4Addr::new(127, 0, 0, 1);
///     let port = 12345;
///
///     // The following lines are equivalent modulo possible "localhost" name
///     // resolution differences
///     let tcp_s = TcpStream::connect(SocketAddrV4::new(ip, port));
///     let tcp_s = TcpStream::connect((ip, port));
///     let tcp_s = TcpStream::connect(("127.0.0.1", port));
///     let tcp_s = TcpStream::connect(("localhost", port));
///     let tcp_s = TcpStream::connect("127.0.0.1:12345");
///     let tcp_s = TcpStream::connect("localhost:12345");
///
///     // TcpListener::bind(), UdpSocket::bind() and UdpSocket::send_to()
///     // behave similarly
///     let tcp_l = TcpListener::bind("localhost:12345");
///
///     let mut udp_s = UdpSocket::bind(("127.0.0.1", port)).unwrap();
///     udp_s.send_to(&[7], (ip, 23451)).unwrap();
///
///
///
///
///
// some bytes, in a vector
//    let sparkle_heart = vec![240, 159, 146, 150];
//    let sparkle_heart = unsafe {
//        String::from_utf8_unchecked(sparkle_heart)
//    };
//    println!("{:?}",sparkle_heart);
//    let s = String::from("张伯雨");
//    let bytes = s.into_bytes();
//    {
//
//        let n=String::from_utf8(bytes);
//        match n {
//            Ok( t)=>{
//                println!("t={}",t);
//            },
//            Err(e)=>{
//                println!("e==={:?}",e);
//            },
//        }
//    }
//   println!("bytes==={:?}",bytes);
//
//    assert_eq!("💖", sparkle_heart);
//
//    return ;
fn main() {



    match process() {

        Ok(t)=>{
            println!("{:?}",t);
        },
        Err(e)=>{
            println!("{:?}",e.kind());
        },
    }

    match process_upgrade() {

        Ok(t)=>{
            println!("{:?}",t);
        },
        Err(e)=>{
            println!("{:?}",e.kind());
        },
    }

}

fn handle_client(stream: &mut  TcpStream) {
    stream.write(b"GET /  -----------ocean-------------     HTTP/1.0\n\n").ok().unwrap();
    let mut s = Vec::new();
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
        Ok( t)=>{

            println!("--------{:?}",t);
        },
        Err(e)=>{println!("------===={:?}",e);},
    }
    println!("===={:?}",555588);
}
fn process() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        //每个连接一个线程,没有限制开辟线程的数量   !!! 这是一个致命缺陷
        thread::spawn(move || {
            println!("====thread==  ");
            handle_client(&mut stream.unwrap());
            println!("====thread=2=  ");
        });
    }
    Ok(())
}

fn process_upgrade() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    let thread_pool = Pool::with_capacity(64, 64);
    for stream in listener.incoming() {
        thread_pool.spawn(move || {
            println!("====thread==  ");
            handle_client(&mut stream.unwrap());
            println!("====thread=2=  ");
        });
    }
    thread::sleep(Duration::from_secs(9999999999999999u64));
    Ok(())
}
//==================================================================================

//    while let Err(e) = stream.read(&mut buf) {
//        println!("====e==  {:?}",e.kind());
//        break ;
//    }

//===================================================================================

//println!("{:?}",111111);
//let  mut s=String::new();
//let r=stream.read_to_string(&mut s);  //在这里会阻塞 和 read_to_end()  一样

//let  mut array=[0u8; 128];
//let r=stream.read(&mut array[..]);

//    let mut buffer = [0; 512];
//    let r=stream.read(&mut buffer);

//=============================================================

//loop{
//println!("{:?}",333333);
//const CHUNK_SIZE: usize = 4096;
//let mut buf = [0; CHUNK_SIZE];
//let r=stream.read(&mut buf);
//println!("{:?}",44444);
//if Ok(t)==r{
//println!("{:?}",555555);
//s.extend_from_slice(s);
//// println!("content:::::{:?}\n",String::from_utf8_lossy(&buffer[..]));
//// println!("content:::::{:?}\n" ,String::from_utf8(s).unwrap());
//
//}else if Err(e)==r {
//println!("{:?}",555577);
//println!("====e==  {:?}",e.kind());
//}else {
//println!("I don't know the mistake  !!!!!!! ");
//}
//}