use crate::socket_server::frame::Frame;
use crossbeam_channel::Sender;
use std::io::BufReader;
use std::net::TcpListener;  
use std::thread;

pub fn run_server(tx: Sender<Frame>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let tx = tx.clone();
                thread::spawn(move || {
                    let reader = BufReader::new(stream);
                    let frame: Frame = serde_json::from_reader(reader).unwrap();
                    tx.send(frame).unwrap();
                });
            }
            Err(_) => {
                eprintln!("Connection failed");
            }
        }
    }
}