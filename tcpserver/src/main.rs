use std::io::{Read,Write};
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");
    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        println!("Connection establish!");
        let mut buffer = [0;1024];
        //将stream里的内容读到buffer
        stream.read(&mut buffer).unwrap();
        //将stream里的内容写到buffer
        stream.write(&mut buffer).unwrap();
    }
}
