use macroquad::{prelude::*, telemetry};
use std::collections::VecDeque;

pub struct Debug {
    display: bool,
    page: i8,
    frame_times: VecDeque<f32>,
    avg_fps: i32,
    avg_frame_time: f32,
    sample_size: usize,
}

impl Debug {
    pub fn new() -> Self {
        let sample_size = 120; // Moyenne sur les 120 dernières trames (environ 2 secondes à 60 FPS)
        Self {
            display: false,
            page: 0,
            frame_times: VecDeque::with_capacity(sample_size),
            avg_fps: 0,
            avg_frame_time: 0.0,
            sample_size,
        }
    }

    pub fn update(&mut self) {
        let frame_time = get_frame_time();
        self.frame_times.push_back(frame_time);

        if self.frame_times.len() > self.sample_size {
            self.frame_times.pop_front();
        }

        let total_time: f32 = self.frame_times.iter().sum();
        if !self.frame_times.is_empty() {
            self.avg_frame_time = total_time / self.frame_times.len() as f32;
            self.avg_fps = (1.0 / self.avg_frame_time) as i32;
        }

        if is_key_pressed(KeyCode::F3) {
            
            // test if others keys are down, for pages
            if is_key_down(KeyCode::Kp0) {
                self.page = 0;
            } else if is_key_down(KeyCode::Kp1) {
                self.page = 1;
            }
            self.display = !self.display;

            // Save some ressources
            if self.display {
                telemetry::enable();
            } else {
                telemetry::disable(); 
            }
            
        }
    }

    pub fn draw(&self){
        if !self.display {return;}
        draw_text_ex("Debug menu - Press F3 + Numpad to choose a page", 1.0, 20.0, TextParams::default());
        match self.page {
            1 => {telemetry_infos(self)} // Telemetry
            0 | _ => {basic_infos(self)} // Basic infos
        }
    }
}

pub fn basic_infos(infos: &Debug) {
    draw_text_ex(format!("FPS : {}", get_fps()).as_str(), 1.0, 35.0, TextParams::default());
    draw_text_ex(format!("AVG FPS : {}", infos.avg_fps).as_str(), 1.0, 50.0, TextParams::default());
    draw_text_ex(format!("Frame Time : {:.2}ms", get_frame_time() * 1000.0).as_str(), 1.0, 65.0, TextParams::default());
    draw_text_ex(format!("AVG Frame Time : {:.2}ms", infos.avg_frame_time * 1000.0).as_str(), 1.0, 80.0, TextParams::default());
}
pub fn telemetry_infos(_infos: &Debug) {
    draw_text_ex(format!("Textures count : {:?}", telemetry::textures_count()).as_str(), 1.0, 35.0, TextParams::default());
}