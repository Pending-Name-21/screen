use colored::Colorize;
use dotenv_codegen::dotenv;
use nannou::event::WindowEvent;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::input_device_monitor::event_caster::IEventCaster;
use crate::input_device_monitor::my_event::event_type::event_type::EventType;
use crate::input_device_monitor::sender::IEventSender;
use crate::log_handler::{run_in_terminal, run_in_terminal_or_not};

pub const SOCKET_SERVER_PATH: &str = dotenv!("SOCKET_SERVER_PATH");
pub struct SocketClientSender {
    stream: UnixStream,
    caster: Box<dyn IEventCaster + Send>,
}

impl SocketClientSender {
    pub fn new(socket_path: &str, caster: Box<dyn IEventCaster>) -> std::io::Result<Self> {
        match UnixStream::connect(socket_path) {
            Ok(stream) => Ok(Self { stream, caster }),
            Err(err) => {
                run_in_terminal_or_not(
                    || {
                        println!(
                            "{} {}",
                            "Failed to connect to the socket server:".bold().red(),
                            err
                        )
                    },
                    || println!("Failed to connect to the socket server: {}", err),
                );
                Err(err)
            }
        }
    }
}

impl IEventSender for SocketClientSender {
    fn send_event(&mut self, event: &WindowEvent) {
        let buf = self.caster.cast_event(event);
        self.stream.write_all(&buf).unwrap();
        let event_type: EventType = event.clone().into();
        run_in_terminal(|| {
            println!("{} : {}", "✓ Sent".bold().green(), event_type);
        });
    }
}

#[cfg(test)]
mod tests {
    use nannou::event::Key;
    use tempfile::TempDir;

    use crate::input_device_monitor::event_caster::clone_caster::clone_caster::CloneCaster;
    use crate::input_device_monitor::my_event::serializable_clone::MyKey;
    use crate::input_device_monitor::my_event::serializable_clone::MyWindowEvent;

    use super::*;
    use std::io::Read;
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
                    Ok((mut socket, _)) => {
                        let mut buf = [0; 1024];
                        loop {
                            match socket.read(&mut buf) {
                                Ok(0) => break,
                                Ok(n) => {
                                    let buf_str = String::from_utf8_lossy(&buf[..n]).to_string();
                                    let event: MyWindowEvent =
                                        serde_json::from_str(&buf_str).unwrap();
                                    assert_eq!(event, MyWindowEvent::MyKeyPressed(MyKey::MyA));
                                    break;
                                }
                                Err(_) => break,
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        let caster = Box::new(CloneCaster);
        let mut client_sender = SocketClientSender::new(socket_path_str, caster).unwrap();

        let test_event = WindowEvent::KeyPressed(Key::A);
        client_sender.send_event(&test_event);

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
    }
}
