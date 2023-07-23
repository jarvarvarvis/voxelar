//! This is a module that provides frame count, FPS and frame time measurement functionality in the
//! form of a `FrameTimeManager` struct.
//!
//! # Examples
//!
//! ```ignore
//! let ctx = Voxelar::new()?;
//! let mut frame_time_manager = FrameTimeManager::new(&ctx);
//!
//! while running {
//!     // Get FrameTimeManager information
//!     let fps = frame_time_manager.fps();
//!     let delta_time = frame_time_manager.delta_time();
//!
//!     println!("FPS: {}", fps);
//!
//!     // Do your rendering here...
//!
//!     // Update the `FrameTimeManager`
//!     frame_time_manager.update(&ctx);
//! }
//! ```

use crate::Voxelar;

/// This is a struct that stores information about the amount of frames drawn, the FPS measurement
/// and delta time.
pub struct FrameTimeManager {
    // FPS-related information
    next_frame_time_stamp: f64,
    last_frame_time_stamp: f64,
    fps: f64,

    // Frame counters
    frames: u64,
    total_frames: u128,
}

impl FrameTimeManager {
    /// Create a new `FrameTimeManager`.
    ///
    /// This function takes a `Voxelar` value and queries the start time for the FPS and the delta
    /// time measurement, so the context needs to be initialized.
    pub fn new(context: &Voxelar) -> Self {
        Self {
            next_frame_time_stamp: 0.0,
            last_frame_time_stamp: context.current_time(),
            fps: 0.0,

            frames: 0,
            total_frames: 0,
        }
    }

    /// Updates the `FrameTimeManager` after the frame has been drawn.
    ///
    /// Specifically, this function updates the FPS in at-least-one-second intervals,
    /// the delta time and the frame counts.
    pub fn update(&mut self, context: &Voxelar) {
        // Measure FPS
        self.next_frame_time_stamp = context.current_time();
        let frame_time_diff = self.frame_time_diff();
        if frame_time_diff > 1.0 || self.frames == 0 {
            self.fps = self.frames as f64 / frame_time_diff;
            self.last_frame_time_stamp = self.next_frame_time_stamp;
            self.frames = 0;
        }

        // Update frame counts
        self.frames += 1;
        self.total_frames += 1;
    }

    pub(crate) fn frame_time_diff(&self) -> f64 {
        self.next_frame_time_stamp - self.last_frame_time_stamp
    }

    /// Returns the total number of frames that the `FrameTimeManager` has counted.
    pub fn total_frames(&self) -> u128 {
        self.total_frames
    }

    /// Resets the total number of frames to 0.
    pub fn reset_total_frames(&mut self) {
        self.total_frames = 0;
    }

    /// Returns the current FPS value.
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Returns the average time per frame in floating point seconds.
    ///
    /// A value of 1.0 returned from this function is equal to one second.
    pub fn frame_time(&self) -> f64 {
        1.0 / self.fps
    }
}
