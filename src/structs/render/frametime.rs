use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use imgui::Ui;

use crate::structs::opengl::frames::SimpleFrame;

pub struct FrameTime {
    pub frametime: Option<Duration>,
    pub average_frametime: Option<Duration>,
    pub fastest_frametime: Duration,
    pub slowest_frametime: Duration,

    pub average_frametime_queue: VecDeque<Duration>,
    pub average_frametime_max_tracking: u32,

    pub start_tracking_time: Instant,
}

impl FrameTime {
    pub fn build(average_frametime_max_tracking: u32) -> FrameTime {
        FrameTime {
            frametime: None,
            average_frametime: None,
            fastest_frametime: Duration::MAX,
            slowest_frametime: Duration::ZERO,

            average_frametime_queue: VecDeque::new(),
            average_frametime_max_tracking,

            start_tracking_time: Instant::now(),
        }
    }

    pub fn start_tracking(&mut self) {
        self.start_tracking_time = Instant::now();
    }

    pub fn update(&mut self) {
        let end_tracking_time = Instant::now();

        self.frametime = Some(end_tracking_time - self.start_tracking_time);

        self.average_frametime_queue
            .push_back(self.frametime.unwrap());

        self.average_frametime = Some(
            self.average_frametime_queue
                .iter()
                .sum::<Duration>()
                .div_f32(self.average_frametime_queue.len() as f32),
        );

        if self.average_frametime_queue.len() > self.average_frametime_max_tracking as usize {
            self.average_frametime_queue.pop_front();
        }

        if self.frametime.expect("Setting frametime failed") < self.fastest_frametime {
            self.fastest_frametime = self
                .frametime
                .expect("Unwrapping frametime to set fastest_frametime failed.");
        } else if self.frametime.expect("Setting frametime failed") > self.slowest_frametime {
            self.slowest_frametime = self
                .frametime
                .expect("Unwrapping frametime to set slowest_frametime failed.");
        }
    }

    pub fn reset_frametimes(&mut self) {
        self.frametime = None;
        self.fastest_frametime = Duration::MAX;
        self.slowest_frametime = Duration::ZERO;
        self.average_frametime_queue.clear();
        self.average_frametime = None;
    }

    pub fn draw_frametime_info(&mut self, ui: &Ui, frame: &SimpleFrame) {
        ui.window("Frametime Status (one behind)").build(|| {
            ui.text(format!("Render Resolution: ({}, {})", frame.fbo_width, frame.fbo_height));
            ui.text(format!("Frame Time: {:?}", self.frametime));
            if !self.average_frametime_queue.is_empty() {
                ui.text(format!("Average Frame Time: {:?}", self.average_frametime));
            } else {
                ui.text("Frame Collection Empty or Not Updated");
            }
            ui.text(format!("Fastest Frame Time: {:?}", self.fastest_frametime));
            ui.text(format!("Slowest Frame Time: {:?}", self.slowest_frametime));
            if ui.button("Reset") {
                self.reset_frametimes();
            }
        });
    }
}
