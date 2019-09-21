#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate log;
use rand::rngs::OsRng;
use rand::Rng;
use raqote::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{HtmlCanvasElement, ImageData};

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
    w: f32,
    h: f32,
    context: web_sys::CanvasRenderingContext2d,
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
            w: w as f32,
            h: h as f32,
            context,
        }
    }

    pub fn render(&mut self) {
        let mut dt = DrawTarget::new(self.w as i32, self.h as i32);

        for _ in 0..2000 {
            let color = SolidSource {
                r: OsRng.gen::<u8>(),
                g: OsRng.gen::<u8>(),
                b: OsRng.gen::<u8>(),
                a: OsRng.gen::<u8>(),
            };
            let x = OsRng.gen::<f32>() * self.w;
            let y = OsRng.gen::<f32>() * self.h;
            let radius = OsRng.gen::<f32>() * 25.;
            let mut pb = PathBuilder::new();
            pb.move_to(x, y);
            pb.arc(x - radius, y, radius, 0., 2. * 3.14159);
            pb.close();
            let path = pb.finish();
            dt.fill(&path, &Source::Solid(color), &DrawOptions::new());
        }

        let mut pixel_data = dt.get_data_u8_mut();

        // Convert raw pixel_data to ImageData object
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut pixel_data),
            self.w as u32,
            self.h as u32,
        );

        // Place image_data onto canvas
        let _ = self.context.put_image_data(&image_data.unwrap(), 0.0, 0.0);
    }
}
