use ferray::random::{default_rng_seeded};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, PremultipliedColorU8, Stroke, Transform};
use anyhow::{Result, bail};
use num_traits::float::FloatConst;
use noise::{NoiseFn, Perlin, Seedable};

fn prob(x: f32, y: f32) -> f32 {
    x * y
}

fn draw_strand(pixmap: &mut Pixmap, length: f32, width: f32, x: f32, y: f32, angle: f32, angle_delta: f32, color: Color) -> Result<()> {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;
    let w = length;
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
    let perlin = Perlin::new(1);
    for i in 0..5000 {
        let (mut x, mut y) = (0.0, 0.0);
        while true {
            (x, y) = (rng.next_f32(), rng.next_f32());
            let p = prob(x, y);
            let e = rng.next_f32();
            if e < p {
                break;
            }
        }
        let angle = rng.next_f32() * 2.0 * std::f32::consts::PI;
        let angle_delta = rng.next_f32() - 0.5;
        draw_strand(&mut pixmap, 0.01, 0.001, x, y, angle, angle_delta, Color::WHITE)?;
    }
    pixmap.save_png("out.png")?;

    Ok(())
}
