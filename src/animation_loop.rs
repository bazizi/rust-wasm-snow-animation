use crate::particle::Particle;
use crate::wasm_helpers::*;
use wasm_bindgen::prelude::*;

pub struct AnimationLoop {
    particles: Vec<Option<Particle>>,
}

const NUM_PARTICLES: usize = 500;

impl AnimationLoop {
    pub fn new() -> Self {
        Self { particles: vec![] }
    }

    pub fn update(&mut self) {
        if self.particles.len() < NUM_PARTICLES {
            self.particles.push(Some(Particle::new()));
        }

        let canvas_context = canvas_context();
        canvas_context.set_fill_style(&JsValue::from_str("darkblue"));
        canvas_context.fill_rect(0., 0., canvas_width(), canvas_height());

        for i in 0..self.particles.len() {
            let particle = self.particles[i].as_mut().unwrap();
            particle.update();
            particle.render();
        }

        // get rid of self.particles that have hit their max iteration
        self.particles = self
            .particles
            .iter_mut()
            .filter(|particle| {
                return !particle.as_ref().unwrap().reached_max_iteration();
            })
            .map(|particle| particle.take())
            .collect();
    }
}
