#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate log;
use piet::kurbo::{Circle, Rect};
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

#[wasm_bindgen]
pub struct RenderEnv {
    w: f64,
    h: f64,
    context: web_sys::CanvasRenderingContext2d,
    window: web_sys::Window,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl RenderEnv {
    pub fn new() -> Self {
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
        let context = canvas
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

        RenderEnv {
            w,
            h,
            context,
            window,
        }
    }

    pub fn render(&mut self) {
        let mut piet_context = WebRenderContext::new(&mut self.context, &self.window);
        // piet_context.clear(Color::WHITE); // Not implemented yet
        let canvas_rect = Rect::new(0., 0., self.w, self.h);
        let brush = piet_context.solid_brush(Color::WHITE);
        piet_context.fill(&canvas_rect, &brush);
        for _ in 0..2000 {
            let brush = piet_context.solid_brush(Color::rgba(
                OsRng.gen::<f64>(),
                OsRng.gen::<f64>(),
                OsRng.gen::<f64>(),
                OsRng.gen::<f64>(),
            ));
            let pt = (OsRng.gen::<f64>() * self.w, OsRng.gen::<f64>() * self.h);
            let radius = OsRng.gen::<f64>() * 25.;
            let shape = Circle::new(pt, radius);
            // let shape = Rect::new(pt.0-radius, pt.1-radius, pt.0+radius, pt.1+radius);
            piet_context.fill(&shape, &brush);
        }
        piet_context.finish().unwrap();
    }
}
