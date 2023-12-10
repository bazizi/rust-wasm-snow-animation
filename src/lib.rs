mod utils;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Math::random;

use web_sys::console::log_1 as log;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn init_rust() {
    let window = window();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas");
    let canvas = canvas.unwrap().dyn_into::<HtmlCanvasElement>().unwrap();

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
    const MAX_ITERATION: u64 = 100;

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
            let size = random() * 4.;
            Particle {
                canvas_context,
                canvas_dimentions: canvas_dimentions,
                position: ((canvas_dimentions.0 - size) * random(), 0.),
                size: (size, size),
                velocity: (random() * 1. + 0.001, random() * 1. + 0.1),
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
        if particles.borrow().len() < 500 {
            particles.borrow_mut().push(Some(Particle::new(
                Rc::clone(&canvas_context),
                (canvas_width, canvas_height),
            )));
        }

        {
            let canvas_context = canvas_context.borrow_mut();
            canvas_context.set_fill_style(&JsValue::from_str("blue"));
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

        let part: Vec<Option<Particle>> = particles
            .borrow_mut()
            .iter_mut()
            .filter(|particle| {
                return !particle.as_ref().unwrap().reached_max_iteration();
            })
            .map(|particle| particle.take())
            .collect();

        *particles.borrow_mut() = part;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
