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
    use crate::input_device_monitor::event_caster::clone_caster::clone_caster::CloneCaster;
    use crate::input_device_monitor::event_caster::flatbuffer_caster::flatbuffer_caster::FlatBufferCaster;
    use crate::input_device_monitor::my_event::flatbuffer::Event;
    use crate::input_device_monitor::my_event::serializable_clone::{
        MyKey, MyMouseButton, MyWindowEvent,
    };
    use nannou::event::{Key, MouseButton, WindowEvent};
    use tempfile::TempDir;

    use super::*;
    use std::io::Read;
    use std::os::unix::net::UnixListener;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;

    fn start_listener<F>(
        socket_path_str: &str,
        handle_event: F,
    ) -> (Arc<AtomicBool>, thread::JoinHandle<()>)
    where
        F: Fn(&[u8]) + Send + 'static,
    {
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
                                    handle_event(&buf[..n]);
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
        (running, handle)
    }

    fn create_client_sender(
        socket_path_str: &str,
        caster: Box<dyn IEventCaster + Send>,
    ) -> SocketClientSender {
        SocketClientSender::new(socket_path_str, caster).unwrap()
    }

    #[test]
    fn test_socket_client_sender_keyboard_event_with_clone_caster() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let socket_path_str = socket_path.to_str().unwrap();

        let (running, handle) = start_listener(socket_path_str, |buf| {
            let buf_str = String::from_utf8_lossy(buf).to_string();
            let event: MyWindowEvent = serde_json::from_str(&buf_str).unwrap();
            assert_eq!(event, MyWindowEvent::MyKeyPressed(MyKey::MyA));
        });

        let caster = Box::new(CloneCaster);
        let mut client_sender = create_client_sender(socket_path_str, caster);

        let test_event = WindowEvent::KeyPressed(Key::A);
        client_sender.send_event(&test_event);

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
    }

    #[test]
    fn test_socket_client_sender_mouse_event_with_clone_caster() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let socket_path_str = socket_path.to_str().unwrap();

        let (running, handle) = start_listener(socket_path_str, |buf| {
            let buf_str = String::from_utf8_lossy(buf).to_string();
            let event: MyWindowEvent = serde_json::from_str(&buf_str).unwrap();
            assert_eq!(event, MyWindowEvent::MyMousePressed(MyMouseButton::MyLeft));
        });

        let caster = Box::new(CloneCaster);
        let mut client_sender = create_client_sender(socket_path_str, caster);

        let test_event = WindowEvent::MousePressed(MouseButton::Left);
        client_sender.send_event(&test_event);

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
    }

    #[test]
    fn test_socket_client_sender_keyboard_event_with_flatbuffer_caster() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let socket_path_str = socket_path.to_str().unwrap();

        let (running, handle) = start_listener(socket_path_str, |buf| {
            let event = flatbuffers::root::<Event>(buf).unwrap();
            match event.keyboard() {
                Some(keyboard_event) => {
                    assert_eq!(keyboard_event.key().unwrap(), "A");
                    assert_eq!(keyboard_event.type_().unwrap(), "KeyPressed");
                }
                None => panic!("No keyboard event received"),
            }
        });

        let caster = Box::new(FlatBufferCaster);
        let mut client_sender = create_client_sender(socket_path_str, caster);

        let test_event = WindowEvent::KeyPressed(Key::A);
        client_sender.send_event(&test_event);

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
    }

    #[test]
    fn test_socket_client_sender_mouse_event_with_flatbuffer_caster() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let socket_path_str = socket_path.to_str().unwrap();

        let (running, handle) = start_listener(socket_path_str, |buf| {
            let event = flatbuffers::root::<Event>(buf).unwrap();
            match event.mouse() {
                Some(mouse_event) => {
                    assert_eq!(mouse_event.button().unwrap(), "Left");
                    assert_eq!(mouse_event.type_().unwrap(), "MousePressed");
                }
                None => panic!("No mouse event received"),
            }
        });

        let caster = Box::new(FlatBufferCaster);
        let mut client_sender = create_client_sender(socket_path_str, caster);

        let test_event = WindowEvent::MousePressed(MouseButton::Left);
        client_sender.send_event(&test_event);

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
    }
}
