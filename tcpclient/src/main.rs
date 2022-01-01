use std::io::{Read,Write};
use std::net::TcpStream;
use std::str;
fn main() {
    //创建到指定地址的连接
    let mut  stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("hello,tom".as_bytes()).unwrap();
    let mut buffer=[0;5];
    stream.read(&mut buffer).unwrap();
    print!("Respone from server:{:?}",str::from_utf8(&buffer).unwrap());
}
