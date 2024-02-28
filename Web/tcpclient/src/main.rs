//第1章 Web/tcpclient/main.rs

use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    //连接服务器，指明地址和端口
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    //向服务器写入
    stream.write("Hello".as_bytes()).unwrap();
    //缓冲区
    let mut buffer = [0; "Hello".len()];
    //读取服务器的应答
    stream.read(&mut buffer).unwrap();

    println!("Response from server:{:?}", str::from_utf8(&buffer).unwrap());
}
