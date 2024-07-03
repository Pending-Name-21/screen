use crossbeam_channel::Receiver;
use std::collections::VecDeque;
use crate::coffee_time::output::frame::frame_generated::coffee_time::output::frame::root_as_frame;
use crate::coffee_time::output::label::label_generated::coffee_time::output::label::root_as_label;
use crate::socket_server::frame::Frame;
use crate::socket_server::deserialization::deserialize_frame;

const FRAME_BUFFER_SIZE: usize = 8;

pub struct FrameReceiver {
    receiver: Receiver<Vec<u8>>,
    frame_buffer: VecDeque<Frame>,
}

impl FrameReceiver {
    pub fn new(receiver: Receiver<Vec<u8>>) -> Self {
        Self {
            receiver,
            frame_buffer: VecDeque::with_capacity(FRAME_BUFFER_SIZE),
        }
    }

    pub fn receive_frames(&mut self) {
        while let Ok(buffer) = self.receiver.try_recv() {
            if let Ok(flat_frame) = root_as_frame(&buffer) {
                match deserialize_frame(flat_frame) {
                    Ok(frame) => {
                        if self.frame_buffer.len() >= FRAME_BUFFER_SIZE {
                            self.frame_buffer.pop_front();
                        }
                        self.frame_buffer.push_back(frame);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse frame: {:?}", e);
                    }
                }
            } else if let Ok(flat_label) = root_as_label(&buffer) {
                println!("Received a label: {:?}", flat_label);
            } else {
                eprintln!("Failed to parse buffer as either Frame or Label");
            }
        }
    }

    pub fn get_next_frame(&mut self) -> Option<Frame> {
        self.frame_buffer.pop_front()
    }
}
