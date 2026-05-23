use ferray::random::{default_rng_seeded};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, PremultipliedColorU8, Stroke, Transform};
use anyhow::{Result, bail};
use num_traits::float::FloatConst;

fn draw_strand(pixmap: &mut Pixmap, x: f32, y: f32, angle: f32, angle_delta: f32, color: Color) -> Result<()> {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;
    let w = 0.02;
    let width = 0.001;
    let (x1, y1) = (x + w * f32::cos(angle), y + w * f32::sin(angle));
    let (x2, y2) = (x1 + w * f32::cos(angle + angle_delta), y1 + w * f32::sin(angle + angle_delta));
    let Some(path) = ({
        let mut pb = PathBuilder::new();
        pb.move_to(x, y);
        //pb.line_to(x1, y1);
        pb.quad_to(x1, y1, x2, y2);
        pb.finish()
    }) else {
        bail!("could not make path");
    };
    let stroke = Stroke { width, ..Default::default() };
    pixmap.stroke_path(
        &path,
        &paint,
        &stroke,
        Transform::from_scale(pixmap.width() as f32, pixmap.width() as f32),
        None,
    );
    Ok(())
}

fn main() -> Result<()> {
    let mut rng = default_rng_seeded(42);
    let scale = 1.0;
    let width = 1000;
    let height = 1000;
    let Some(mut pixmap) = Pixmap::new(
        ((width as f32) * scale) as u32,
        ((height as f32) * scale) as u32,
    ) else {
        bail!("pixmap failed");
    };
    pixmap.fill(Color::BLACK);
    for i in 0..1000 {
        let (x, y) = (rng.next_f32(), rng.next_f32());
        let angle = rng.next_f32() * 2.0 * std::f32::consts::PI;
        let angle_delta = rng.next_f32() - 0.5;
        draw_strand(&mut pixmap, x, y, angle, angle_delta, Color::WHITE)?;
    }
    pixmap.save_png("out.png")?;

    Ok(())
}
