use crate::socket_server::frame::Frame;
use crossbeam_channel::Sender;
use std::io::BufReader;
use std::net::TcpListener;  
use std::thread;
use dotenv::dotenv;
use std::env;

pub fn run_server(tx: Sender<Frame>) {
    dotenv().ok();
    let address: String = env::var("ADDRESS").expect("ADDRESS must be set");
    let listener = TcpListener::bind(address).unwrap();
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