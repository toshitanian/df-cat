use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let ip = "127.0.0.1";
    let port = 9123;
    let bind_addr = format!("{}:{}", ip, port);
    let listener: Result<TcpListener, _> = TcpListener::bind(bind_addr);

    // Try to bind local address
    let listener = match listener {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut buf = [0; 1024];
            let mut stream = stream.unwrap();
            println!("reading...");
            println!("Received {} bytes", stream.read(&mut buf).unwrap());
            let res = buf.iter().map(|&s| s as char).collect::<String>();
            //            let converted: String = String::from_utf8(buf.to_vec()).unwrap();
            let tcp_res = format!("{}\r\n", res);
            stream.write(tcp_res.as_bytes()).unwrap();
        });
    }
}
