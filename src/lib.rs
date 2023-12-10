mod utils;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Math::random;

use web_sys::console::log_1 as log;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use utils::set_panic_hook;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn create_canvas() -> HtmlCanvasElement {
    let canv = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canv.set_width(window().inner_width().unwrap().as_f64().unwrap() as u32);
    canv.set_height(window().inner_height().unwrap().as_f64().unwrap() as u32 - 10);

    canv
}

#[wasm_bindgen]
pub fn init_rust() {
    set_panic_hook();
    let canvas = create_canvas();
    document().body().unwrap().append_child(&canvas).unwrap();

    let canvas_context = Rc::new(RefCell::new(
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap(),
    ));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let canvas_width: f64 = canvas.width().into();
    let canvas_height: f64 = canvas.height().into();

    const WIDTH: f64 = 50.;
    const HEIGHT: f64 = 50.;
    const MIN_VELOCITY: f64 = 5.;
    const MIN_SIZE: f64 = 10.;
    const MAX_ITERATION: u64 = 100;
    const NUM_PARTICLES: usize = 500;

    #[derive(Debug)]
    struct Particle {
        canvas_context: Rc<RefCell<CanvasRenderingContext2d>>,
        canvas_dimentions: (f64, f64),
        size: (f64, f64),
        position: (f64, f64),
        velocity: (f64, f64),
        iteration: u64,
    }

    impl Particle {
        pub fn new(
            canvas_context: Rc<RefCell<CanvasRenderingContext2d>>,
            canvas_dimentions: (f64, f64),
        ) -> Self {
            let size = random() * MIN_SIZE;
            Particle {
                canvas_context,
                canvas_dimentions: canvas_dimentions,
                position: ((canvas_dimentions.0 - size) * random(), 0.),
                size: (size, size),
                velocity: (random() * 1. + 0.001, random() * 5. + MIN_VELOCITY),
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
                self.iteration += 1;
            }
        }

        pub fn render(&mut self) {
            let canvas_context = self.canvas_context.borrow_mut();
            canvas_context.set_fill_style(&JsValue::from_str("white"));
            canvas_context.fill_rect(self.position.0, self.position.1, self.size.0, self.size.1);
        }

        pub fn reached_max_iteration(&self) -> bool {
            return self.iteration >= MAX_ITERATION;
        }
    }

    let particles = Rc::new(RefCell::new(Vec::new()));

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if particles.borrow().len() < NUM_PARTICLES {
            particles.borrow_mut().push(Some(Particle::new(
                Rc::clone(&canvas_context),
                (canvas_width, canvas_height),
            )));
        }

        {
            let canvas_context = canvas_context.borrow_mut();
            canvas_context.set_fill_style(&JsValue::from_str("darkblue"));
            canvas_context.fill_rect(0., 0., canvas_width, canvas_height);
        }

        {
            let mut particles = particles.borrow_mut();
            for i in 0..particles.len() {
                let particle = particles[i].as_mut().unwrap();
                log(&JsValue::from_str(format!("{:#?}", particle).as_str()));
                particle.update();
                particle.render();
            }
        }

        // get rid of particles that have hit their max iteration
        let alive_particles: Vec<Option<Particle>> = particles
            .borrow_mut()
            .iter_mut()
            .filter(|particle| {
                return !particle.as_ref().unwrap().reached_max_iteration();
            })
            .map(|particle| particle.take())
            .collect();

        *particles.borrow_mut() = alive_particles;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
