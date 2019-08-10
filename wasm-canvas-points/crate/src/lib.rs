#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate log;
use piet::kurbo::Circle;
use piet::{Color, RenderContext};
use piet_web::WebRenderContext;
use rand::rngs::OsRng;
use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    let canvas = window
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let mut context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let w = canvas.offset_width() as f64;
    let h = canvas.offset_height() as f64;
    let dpr = window.device_pixel_ratio();
    debug!("device_pixel_ratio: {}", dpr);
    canvas.set_width((w * dpr) as u32);
    canvas.set_height((h * dpr) as u32);
    debug!("canvas width/height: {}x{}", w, h);
    let _ = context.scale(dpr, dpr);

    let mut piet_context = WebRenderContext::new(&mut context, &window);
    for _ in 0..1000 {
        let brush = piet_context.solid_brush(Color::rgba(
            OsRng.gen::<f64>(),
            OsRng.gen::<f64>(),
            OsRng.gen::<f64>(),
            OsRng.gen::<f64>(),
        ));
        let pt = (OsRng.gen::<f64>() * w, OsRng.gen::<f64>() * h);
        let dot = Circle::new(pt, OsRng.gen::<f64>() * 50.);
        piet_context.fill(&dot, &brush);
    }
    piet_context.finish().unwrap();

    Ok(())
}
