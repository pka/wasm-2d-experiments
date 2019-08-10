#[macro_use]
extern crate cfg_if;
use piet::kurbo::{BezPath, Point};
use piet::{Color, RenderContext};
use piet_test::draw_test_picture;
use piet_web::WebRenderContext;
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

fn circle<V: Into<Point>>(center: V, radius: f64, num_segments: usize) -> BezPath {
    let mut path = BezPath::new();
    if num_segments == 0 {
        return path;
    }

    let center = center.into();
    let centerx = center.x;
    let centery = center.y;
    for segment in 0..num_segments {
        let theta = 2.0 * std::f64::consts::PI * (segment as f64) / (num_segments as f64);
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        if segment == 0 {
            path.move_to((x + centerx, y + centery));
        } else {
            let end = (x + centerx, y + centery);
            path.line_to(end);
        }
    }

    path.close_path();
    return path;
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

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

    // let dpr = window.device_pixel_ratio();
    // canvas.set_width((canvas.offset_width() as f64 * dpr) as u32);
    // canvas.set_height((canvas.offset_height() as f64 * dpr) as u32);
    canvas.set_width(3000);
    canvas.set_height(3000);
    let dpr = 20.;
    let _ = context.scale(dpr, dpr);

    let mut piet_context = WebRenderContext::new(&mut context, &window);
    draw_test_picture(&mut piet_context, 2).unwrap();
    let handle_brush = piet_context.solid_brush(Color::rgb8(0x00, 0x00, 0x80));
    for pt in vec![(70.0, 80.0), (140.0, 10.0), (60.0, 10.0), (90.0, 80.0)] {
        let dot = circle(pt, 1.5, 20);
        piet_context.fill(&dot, &handle_brush);
    }
    piet_context.finish().unwrap();

    Ok(())
}
