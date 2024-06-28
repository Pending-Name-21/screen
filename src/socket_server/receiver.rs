use crossbeam_channel::Receiver;
use std::collections::VecDeque;
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
            match deserialize_frame(&buffer) {
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
        }
    }

    pub fn get_next_frame(&mut self) -> Option<Frame> {
        self.frame_buffer.pop_front()
    }
}
