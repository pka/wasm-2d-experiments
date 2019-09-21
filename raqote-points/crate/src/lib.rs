#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate log;
use rand::rngs::OsRng;
use rand::Rng;
use raqote::*;
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
        let mut dt = DrawTarget::new(400, 400);

        let mut pb = PathBuilder::new();
        pb.move_to(100., 10.);
        pb.cubic_to(150., 40., 175., 0., 200., 10.);
        pb.quad_to(120., 100., 80., 200.);
        pb.quad_to(150., 180., 300., 300.);
        pb.close();
        let path = pb.finish();

        let gradient = Source::new_radial_gradient(
            Gradient {
                stops: vec![
                    GradientStop {
                        position: 0.2,
                        color: Color::new(0xff, 0, 0xff, 0),
                    },
                    GradientStop {
                        position: 0.8,
                        color: Color::new(0xff, 0xff, 0xff, 0xff),
                    },
                    GradientStop {
                        position: 1.,
                        color: Color::new(0xff, 0xff, 0, 0xff),
                    },
                ],
            },
            Point::new(150., 150.),
            128.,
            Spread::Pad,
        );
        dt.fill(&path, &gradient, &DrawOptions::new());

        let mut pb = PathBuilder::new();
        pb.move_to(100., 100.);
        pb.line_to(300., 300.);
        pb.line_to(200., 300.);
        let path = pb.finish();

        dt.stroke(
            &path,
            &Source::Solid(SolidSource {
                r: 0x0,
                g: 0x0,
                b: 0x80,
                a: 0x80,
            }),
            &StrokeStyle {
                cap: LineCap::Round,
                join: LineJoin::Round,
                width: 10.,
                miter_limit: 2.,
                dash_array: vec![10., 18.],
                dash_offset: 16.,
            },
            &DrawOptions::new(),
        );

        let _ = dt.write_png("example.png");
    }
}
