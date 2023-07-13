use crate::Voxelar;

pub struct FramesTimer {
    frames: u64,
    total_frames: u128,
    next_time: f64,
    last_time: f64,
    fps: f64,
}

impl FramesTimer {
    pub fn new(context: &Voxelar) -> Self {
        Self {
            frames: 0,
            total_frames: 0,
            next_time: 0.0,
            last_time: context.current_time(),
            fps: 0.0
        }
    }

    pub fn complete_frame(&mut self, context: &Voxelar) {
        self.next_time = context.current_time();

        let time_diff = self.next_time - self.last_time;
        if time_diff > 1.0 || self.frames == 0 {
            self.fps = self.frames as f64 / time_diff;
            self.last_time = self.next_time;
            self.frames = 0;
        }

        self.frames += 1;
        self.total_frames += 1;
    }

    pub fn total_frames(&self) -> u128 {
        self.total_frames
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }
}
