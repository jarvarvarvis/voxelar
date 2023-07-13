use crate::Voxelar;

pub struct TimeManager {
    // Frame counter and FPS
    frames: u64,
    total_frames: u128,
    next_time: f64,
    last_time: f64,
    fps: f64,

    // Delta time
    next_time_for_delta: f64,
    last_time_for_delta: f64,
    delta_time: f64,
}

impl TimeManager {
    pub fn new(context: &Voxelar) -> Self {
        Self {
            frames: 0,
            total_frames: 0,
            next_time: 0.0,
            last_time: context.current_time(),
            fps: 0.0,

            next_time_for_delta: 0.0,
            last_time_for_delta: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn prepare_frame(&mut self, context: &Voxelar) {
        self.next_time_for_delta = context.current_time();
        self.delta_time = self.next_time - self.last_time_for_delta;
        self.last_time_for_delta = self.next_time;
    }

    pub fn complete_frame(&mut self, context: &Voxelar) {
        self.next_time = context.current_time();
        let time_diff = self.frame_time_delta();
        if time_diff > 1.0 || self.frames == 0 {
            self.fps = self.frames as f64 / time_diff;
            self.last_time = self.next_time;
            self.frames = 0;
        }

        self.frames += 1;
        self.total_frames += 1;
    }

    pub fn frame_time_delta(&self) -> f64 {
        self.next_time - self.last_time
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn total_frames(&self) -> u128 {
        self.total_frames
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }
}
