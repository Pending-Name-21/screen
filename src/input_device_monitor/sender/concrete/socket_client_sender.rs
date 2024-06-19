use nannou::event::WindowEvent;
use serde_json;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use crate::input_device_monitor::my_event::MyWindowEvent;
use crate::input_device_monitor::sender::IEventSender;

pub const SOCKET_SERVER_PATH: &str = "/tmp/events-socket.sock";

pub struct SocketClientSender {
    stream: UnixStream,
}

impl SocketClientSender {
    pub fn new(socket_path: &str) -> std::io::Result<Self> {
        let stream = UnixStream::connect(socket_path)?;
        Ok(Self { stream })
    }
}

impl IEventSender for SocketClientSender {
    fn send_event(&mut self, event: &WindowEvent) {
        let my_event: MyWindowEvent = event.clone().into();
        let event_json = serde_json::to_string(&my_event).unwrap();
        writeln!(self.stream, "{}", event_json).unwrap();
        println!("✓ Sent : {}", event_json);
    }
}

#[cfg(test)]
mod tests {
    use nannou::event::Key;
    use tempfile::TempDir;

    use crate::input_device_monitor::my_event::MyKey;

    use super::*;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::os::unix::net::UnixListener;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_socket_client_sender() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let socket_path_str = socket_path.to_str().unwrap();

        let listener = UnixListener::bind(socket_path_str).unwrap();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let handle = thread::spawn(move || {
            while running_clone.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((socket, _)) => {
                        let reader = BufReader::new(socket);
                        for line in reader.lines() {
                            let buf = line.unwrap();
                            let event: MyWindowEvent = serde_json::from_str(&buf).unwrap();
                            assert_eq!(event, MyWindowEvent::MyKeyPressed(MyKey::MyA));
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        let mut client_sender = SocketClientSender::new(socket_path_str).unwrap();

        let test_event = WindowEvent::KeyPressed(Key::A);
        client_sender.send_event(&test_event);

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
    }
}
