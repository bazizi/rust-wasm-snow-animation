mod utils;
use utils::set_panic_hook;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod particle;

mod animation_loop;
use animation_loop::AnimationLoop;

mod wasm_helpers;
use wasm_helpers::*;

#[wasm_bindgen]
pub fn init_rust() {
    set_panic_hook();
    create_canvas();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut animation_loop = AnimationLoop::new();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        animation_loop.update();

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
