use quicksilver::prelude::*;
use quicksilver::{
    geom::Vector,
    graphics::{Color, Mesh, ShapeRenderer},
    lifecycle::{run, Settings, State, Window},
    lyon::{
        math::{point, rect},
        tessellation::basic_shapes::*,
        tessellation::FillOptions,
    },
    Result,
};
use rand;

struct Circles {
    font: Asset<Font>,
}

const W: f32 = 1000.0;
const H: f32 = 500.0;

fn rnd() -> f32 {
    rand::random::<f32>()
}

impl State for Circles {
    fn new() -> Result<Circles> {
        let font = Asset::new(Font::load("mononoki-Regular.ttf"));
        Ok(Circles { font })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let mut mesh = Mesh::new();
        let mut mesh_shape = ShapeRenderer::new(&mut mesh, Color::BLACK);

        let options = FillOptions::tolerance(0.1);
        for _i in 0..400 {
            mesh_shape.set_color(Color {
                r: rnd(),
                g: rnd(),
                b: rnd(),
                a: rnd(),
            });
            let center = point(rnd() * W, rnd() * H);
            fill_circle(center, rnd() * 25., &options, &mut mesh_shape).unwrap();
        }

        mesh_shape.set_color(Color::RED.with_alpha(0.8));
        fill_rounded_rectangle(
            &rect(880.0, 430.0, 100.0, 50.0),
            &BorderRadii {
                top_left: 10.0,
                top_right: 10.0,
                bottom_left: 10.0,
                bottom_right: 10.0,
            },
            &options,
            &mut mesh_shape,
        )
        .unwrap();

        window.clear(Color::WHITE)?;
        window.mesh().extend(&mesh);

        self.font.execute(|font| {
            let fps = format!("{:.1}", window.average_fps());
            let image = font
                .render(&fps, &FontStyle::new(32.0, Color::BLACK))
                .unwrap();
            window.draw(&image.area().with_center((930, 460)), Img(&image));
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<Circles>(
        "quicksilver-points",
        Vector::new(1000, 500),
        Settings {
            multisampling: Some(4),
            ..Settings::default()
        },
    );
}
