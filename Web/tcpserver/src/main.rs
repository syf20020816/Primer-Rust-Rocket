//第1章/tcpserver.rs

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;

fn main() {
    //绑定启动地址和端口
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("server -> 127.0.0.1:3000");
    //持续监听连接
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connect success");
        let mut buffer: [u8; 1024] = [0; 1024];
        //读取客户端写入
        stream.read(&mut buffer).unwrap();
        println!("get msg from client: {:?}", str::from_utf8(&buffer).unwrap());
        //写出，返回客户端
        stream.write(&mut buffer).unwrap();
    }
}
