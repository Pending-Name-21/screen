use crate::socket_server::frame::Frame;
use crossbeam_channel::Sender;
use std::io::BufReader;
use std::os::unix::net::UnixListener;
use rayon::ThreadPool;
use dotenv::dotenv;
use std::env;
use std::fs::remove_file;
use std::io::Read;
use std::sync::Arc;

pub fn run_server(tx: Sender<Frame>, thread_pool: Arc<ThreadPool>) {
    dotenv().ok();
    let socket_path: String = env::var("ADDRESS").expect("ADDRESS must be set");

    let _ = remove_file(&socket_path);

    let listener = UnixListener::bind(&socket_path).expect("Failed to bind socket");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let tx = tx.clone();
                let pool = Arc::clone(&thread_pool);
                pool.spawn(move || {
                    let mut reader = BufReader::new(stream);
                    let mut buffer = Vec::new();
                    match reader.read_to_end(&mut buffer) {
                        Ok(_) => {
                            match serde_json::from_slice::<Frame>(&buffer) {
                                Ok(frame) => {
                                    if tx.send(frame).is_err() {
                                        eprintln!("Failed to send frame to channel");
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Failed to deserialize frame: {}", err);
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to read from stream: {}", err);
                        }
                    }
                });
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }
}