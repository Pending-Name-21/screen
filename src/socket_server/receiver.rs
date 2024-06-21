use crossbeam_channel::Receiver;
use std::collections::VecDeque;
use crate::socket_server::frame::Frame;

const FRAME_BUFFER_SIZE: usize = 8;

pub struct FrameReceiver {
    receiver: Receiver<Frame>,
    frame_buffer: VecDeque<Frame>,
}

impl FrameReceiver {
    pub fn new(receiver: Receiver<Frame>) -> Self {
        Self {
            receiver,
            frame_buffer: VecDeque::with_capacity(FRAME_BUFFER_SIZE),
        }
    }

    pub fn receive_frames(&mut self) {
        while let Ok(frame) = self.receiver.try_recv() {
            if self.frame_buffer.len() >= FRAME_BUFFER_SIZE {
                self.frame_buffer.pop_front();
            }
            self.frame_buffer.push_back(frame);
        }
    }

    pub fn get_next_frame(&mut self) -> Option<Frame> {
        self.frame_buffer.pop_front()
    }
}
