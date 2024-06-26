use crate::socket_server::frame::Frame;
use crate::socket_server::deserialization::deserialize_frame;
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
                    loop {
                        let mut buffer = vec![0; 1024]; 
                        match reader.read(&mut buffer) {
                            Ok(0) => {
                                break;
                            }
                            Ok(n) => {
                                buffer.truncate(n); 
                                match deserialize_frame(&buffer) {
                                    Ok(flat_frame) => {
                                        let _ = tx.send(flat_frame);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to parse frame: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read from socket: {:?}", e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {:?}", e);
            }
        }
    }
}