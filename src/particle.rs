use js_sys::Math::random;

use web_sys::CanvasRenderingContext2d;

use wasm_bindgen::prelude::*;

use crate::wasm_helpers::*;

use crate::wasm_helpers::{canvas_height, canvas_width};

const MIN_VELOCITY_Y: f64 = 5.;
const MIN_VELOCITY_X: f64 = 0.001;
const MAX_SIZE: f64 = 10.;
const MAX_ITERATION: u64 = 100;

#[derive(Debug)]
pub struct Particle {
    canvas_context: CanvasRenderingContext2d,
    canvas_dimentions: (f64, f64),
    size: (f64, f64),
    position: (f64, f64),
    velocity: (f64, f64),
    iteration: u64,
}

impl Particle {
    pub fn new() -> Self {
        let size = random() * MAX_SIZE;
        let canvas_dimentions = (canvas_width(), canvas_height());
        Particle {
            canvas_context: canvas_context(),
            canvas_dimentions,
            position: ((canvas_dimentions.0 - size) * random(), 0.),
            size: (size, size),
            velocity: (
                random() * 1. + MIN_VELOCITY_X,
                random() * 5. + MIN_VELOCITY_Y,
            ),
            iteration: 0,
        }
    }

    pub fn update(&mut self) {
        self.velocity.0 = -self.velocity.0;

        if self.position.0 + self.size.0 <= self.canvas_dimentions.0 {
            self.position.0 += self.velocity.0;
        }

        if self.position.1 + self.size.1 <= self.canvas_dimentions.1 {
            self.position.1 += self.velocity.1;
        } else {
            // Once the particles hit the ground we start incrementing their iteration so we remove them after MAX_ITERATION
            self.iteration += 1;
        }
    }

    pub fn render(&mut self) {
        self.canvas_context
            .set_fill_style(&JsValue::from_str("white"));
        self.canvas_context
            .fill_rect(self.position.0, self.position.1, self.size.0, self.size.1);
    }

    pub fn reached_max_iteration(&self) -> bool {
        return self.iteration >= MAX_ITERATION;
    }
}
