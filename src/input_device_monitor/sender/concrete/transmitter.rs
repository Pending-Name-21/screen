#[cfg(test)]
mod tests {
  use std::os::unix::net::UnixStream;
  use crate::input_device_monitor::sender::concrete::{KeyboardEvent, KeyboardEventArgs};
  use std::io::Write;

  #[test]
  fn send_flattbuffer() {
      let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
      let event_state = builder.create_string("KeyPressed");
      let key_name =  builder.create_string("Return");
      let keyboard_event = KeyboardEvent::create(&mut builder, &KeyboardEventArgs {
        state: Some(event_state),
        name: Some(key_name),
      });
      builder.finish(keyboard_event, None);
      let buf = builder.finished_data();

      let socket = match UnixStream::connect("/tmp/events-socket.sock") {
        Ok(mut sock) => {
          sock.write_all(buf).unwrap();
        },
        Err(e) => {
          println!("Couldn't connect: {e:?}");
          return
        }
      };
  }
}


