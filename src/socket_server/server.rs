use crate::socket_server::frame::Frame;
use crossbeam_channel::Sender;
use std::io::BufReader;
use std::os::unix::net::UnixListener;
use std::thread;
use dotenv::dotenv;
use std::env;
use std::fs::remove_file;

pub fn run_server(tx: Sender<Frame>) {
    dotenv().ok();
    let socket_path: String = env::var("ADDRESS").expect("ADDRESS must be set");

    let _ = remove_file(&socket_path);

    let listener = UnixListener::bind(&socket_path).expect("Failed to bind socket");
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
