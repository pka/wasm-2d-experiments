use quicksilver::{
    geom::{Transform, Vector},
    graphics::{Color, Mesh, ShapeRenderer},
    lifecycle::{run, Settings, State, Window},
    lyon::{
        extra::rust_logo::build_logo_path,
        path::{builder::*, Path},
        tessellation::{FillOptions, FillTessellator},
    },
    Result,
};

struct Circles;

impl State for Circles {
    fn new() -> Result<Circles> {
        Ok(Circles)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Build a Path for the rust logo.
        let mut builder = SvgPathBuilder::new(Path::builder());
        build_logo_path(&mut builder);
        let path = builder.build();

        let filled_logo = {
            let mut logo = Mesh::new();
            let mut logo_shape = ShapeRenderer::new(&mut logo, Color::BLACK);
            logo_shape.set_transform(Transform::scale((3, 3)));
            let mut tessellator = FillTessellator::new();
            tessellator
                .tessellate_path(&path, &FillOptions::tolerance(0.01), &mut logo_shape)
                .unwrap();
            logo
        };

        window.clear(Color::WHITE)?;
        window.mesh().extend(&filled_logo);

        Ok(())
    }
}

fn main() {
    run::<Circles>(
        "quicksilver-points",
        Vector::new(800, 600),
        Settings {
            multisampling: Some(4),
            ..Settings::default()
        },
    );
}
