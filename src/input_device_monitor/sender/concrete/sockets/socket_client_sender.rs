use colored::Colorize;
use dotenv_codegen::dotenv;
use nannou::event::WindowEvent;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::input_device_monitor::event_caster::IEventCaster;
use crate::input_device_monitor::my_event::flatbuffer::{
    Event, EventArgs, Keyboard, KeyboardArgs, Mouse, MouseArgs, Position, PositionArgs,
};
use crate::input_device_monitor::my_event::serializable_clone::{
    get_last_mouse_point, MyWindowEvent,
};
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
        run_in_terminal(|| {
            println!("{}", "✓ Sent".bold().green());
        });
    }
    /* 
    fn send_event(&mut self, event: &WindowEvent) {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let mut type_event = "";
        match event {
            WindowEvent::KeyPressed(key) => {
                type_event = "key pressed";
                let event_state = builder.create_string("KeyPressed");
                let key_name = builder.create_string(&format!("{:?}", key));

                let keyboard_event = Keyboard::create(
                    &mut builder,
                    &KeyboardArgs {
                        type_: Some(event_state),
                        key: Some(key_name),
                    },
                );

                let event = Event::create(
                    &mut builder,
                    &EventArgs {
                        keyboard: Some(keyboard_event),
                        mouse: None,
                    },
                );

                builder.finish(event, None);
            },
            WindowEvent::KeyReleased(key) => {
                type_event = "key released";

                let event_state = builder.create_string("KeyReleased");
                let key_name = builder.create_string(&format!("{:?}", key));

                let keyboard_event = Keyboard::create(
                    &mut builder,
                    &KeyboardArgs {
                        type_: Some(event_state),
                        key: Some(key_name),
                    },
                );

                let event = Event::create(
                    &mut builder,
                    &EventArgs {
                        keyboard: Some(keyboard_event),
                        mouse: None,
                    },
                );

                builder.finish(event, None);
            },
            WindowEvent::MousePressed(button) => {
                type_event = "mouse pressed";

                let point = get_last_mouse_point();
                let x = point.x;
                let y = point.y;
                let event_state = builder.create_string("ButtonPressed");
                let button_name = builder.create_string(&format!("{:?}", button));
                let position = Position::create(&mut builder, &PositionArgs { x, y });

                let mouse_event = Mouse::create(
                    &mut builder,
                    &MouseArgs {
                        type_: Some(event_state),
                        button: Some(button_name),
                        position: Some(position),
                    },
                );

                let event = Event::create(
                    &mut builder,
                    &EventArgs {
                        keyboard: None,
                        mouse: Some(mouse_event),
                    },
                );

                builder.finish(event, None);
            },
            WindowEvent::MouseReleased(button) => {
                type_event = "mouse released";

                let point = get_last_mouse_point();
                let x = point.x;
                let y = point.y;
                let event_state = builder.create_string("MouseReleased");
                let button_name = builder.create_string(&format!("{:?}", button)); // Convert MouseButton enum to string

                let position = Position::create(&mut builder, &PositionArgs { x, y });
                let mouse_event = Mouse::create(
                    &mut builder,
                    &MouseArgs {
                        type_: Some(event_state),
                        button: Some(button_name),
                        position: Some(position),
                    },
                );

                let event = Event::create(
                    &mut builder,
                    &EventArgs {
                        keyboard: None,
                        mouse: Some(mouse_event),
                    },
                );

                builder.finish(event, None);
            },
            _ => {
                println!("Unhandled window event: {:?}", event);
            }
        }

        run_in_terminal(|| {
            println!("{} : {}", "✓ Sent".bold().green(), type_event);
        });

        let buf = builder.finished_data();
        self.stream.write_all(buf).unwrap();
    }
     */
}

#[cfg(test)]
mod tests {
    use nannou::event::Key;
    use tempfile::TempDir;

    use crate::input_device_monitor::event_caster::clone_caster::clone_caster::CloneCaster;
    use crate::input_device_monitor::my_event::serializable_clone::MyKey;

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
                                    let event: MyWindowEvent = serde_json::from_str(&buf_str).unwrap();
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
