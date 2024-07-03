use std::{io::Write, os::unix::net::UnixStream};

use flatbuffers::FlatBufferBuilder;

use crate::coffee_time::output::label::label_generated::coffee_time::output::label::{
    Coord, CoordArgs, Label, LabelArgs,
};

pub struct FlatBufferLabelSerializer;

impl FlatBufferLabelSerializer {
    pub fn serialize_label(&self, text: &str, color: &str, x: i32, y: i32) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();
        let text_offset = builder.create_string(text);
        let color_offset = builder.create_string(color);

        let coord = Coord::create(&mut builder, &CoordArgs { x, y });

        let label = Label::create(
            &mut builder,
            &LabelArgs {
                text: Some(text_offset),
                color: Some(color_offset),
                position: Some(coord),
            },
        );

        builder.finish(label, None);
        builder.finished_data().to_vec()
    }
}

pub struct SocketClient {
    stream: UnixStream,
}

trait SocketSender {
    fn send_event(&mut self, data: Vec<u8>);
}

impl SocketClient {
    pub fn new(socket_path: &str) -> std::io::Result<Self> {
        match UnixStream::connect(socket_path) {
            Ok(stream) => Ok(Self { stream }),
            Err(err) => Err(err),
        }
    }
}

impl SocketSender for SocketClient {
    fn send_event(&mut self, data: Vec<u8>) {
        self.stream.write_all(&data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::{io::Read, os::unix::net::UnixListener, sync::Arc, thread};

    use flatbuffers::{root, FlatBufferBuilder};
    use nannou::event::{Key, WindowEvent};
    use tempfile::TempDir;

    use crate::coffee_time::output::label::label_generated::coffee_time::output::label::{root_as_label, Label};
    use crate::input_device_monitor::my_event::flatbuffer::flatbuffers_events::coffee_time::input_events::root_as_event;
    use crate::input_device_monitor::{
        event_caster::{
            abstractions::IEventSerializer,
            concrete::flatbuffer_caster::flatbuffer_event_serializer::FlatBufferEventSerializer,
        },
        my_event::flatbuffer::Event,
        sender::concrete::sockets::poc_two_schemes::{SocketClient, SocketSender},
    };

    use super::FlatBufferLabelSerializer;

    #[test]
    fn test_socket_client_sender_with_flatbuffer_caster() {
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
                                    if let Ok(event) = root_as_event(&buf[..n]) {
                                        match event.keyboard() {
                                            Some(keyboard_event) => {
                                                assert_eq!(keyboard_event.key().unwrap(), "A");
                                                assert_eq!(
                                                    keyboard_event.type_().unwrap(),
                                                    "KeyPressed"
                                                );
                                            }
                                            None => {
                                                if let Ok(label) =
                                                    root_as_label(&buf[..n])
                                                {
                                                    assert_eq!(label.text().unwrap(), "Test Label");
                                                    assert_eq!(label.color().unwrap(), "Red");
                                                    let position = label.position().unwrap();
                                                    assert_eq!(position.x(), 100);
                                                    assert_eq!(position.y(), 200);
                                                } else {
                                                    println!("Failed to deserialize Label");
                                                }
                                            }
                                        }
                                    } else {
                                        println!("Failed to deserialize Event");
                                    }
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

        let caster = Box::new(FlatBufferEventSerializer);
        let label_serializer = FlatBufferLabelSerializer;
        let mut client_sender = SocketClient::new(socket_path_str).unwrap();

        let test_event = WindowEvent::KeyPressed(Key::A);
        let buf = caster.serialize_event(&test_event);
        client_sender.send_event(buf);

        let label_buf = label_serializer.serialize_label("Test Label", "Red", 100, 200);
        client_sender.send_event(label_buf);

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
    }
}
