use ferray::random::{default_rng_seeded};
use tiny_skia::{Color, Paint, LineCap, PathBuilder, Pixmap, PremultipliedColorU8, Stroke, Transform};
use anyhow::{Result, bail};
use num_traits::float::FloatConst;
use noise::{NoiseFn, Perlin, Seedable};

fn prob(perlin: &Perlin, x: f32, y: f32) -> f32 {
    let p1 = (perlin.get([(x * 3.0) as f64, (y * 3.0) as f64]) as f32 + 1.0) * 0.5;
    let p2 = (perlin.get([(x * 10.0) as f64, (y * 10.0) as f64]) as f32 + 1.0) * 0.5;
    let p3 = (perlin.get([(x * 20.0) as f64, (y * 20.0) as f64]) as f32 + 1.0) * 0.5;
    (p1 + p2 * 0.5 + p3 * 0.25) - 0.3
}

fn draw_trunk(levels: u32, mut pixmap: &mut Pixmap, length: f32, width: f32, x: f32, y: f32, angle: f32, color: Color) -> Result<()> {
    if levels == 0 {
        return Ok(());
    }
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;
    let w = length;
    let (x1, y1) = (x + w * f32::cos(angle), y + w * f32::sin(angle));
    let Some(path) = ({
        let mut pb = PathBuilder::new();
        pb.move_to(x, y);
        pb.line_to(x1, y1);
        pb.finish()
    }) else {
        bail!("could not make path");
    };
    let line_cap = LineCap::Round;
    let stroke = Stroke { width, line_cap, ..Default::default() };
    pixmap.stroke_path(
        &path,
        &paint,
        &stroke,
        Transform::from_scale(pixmap.width() as f32, pixmap.width() as f32),
        None,
    );
    draw_trunk(levels - 1, &mut pixmap, length * 0.8, width * 0.8, x1, y1, angle - 0.5, color)?;
    draw_trunk(levels - 1, &mut pixmap, length * 0.8, width * 0.8, x1, y1, angle + 0.5, color)?;
    Ok(())
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
    pixmap.fill(Color::from_rgba8(102, 70, 75, 255));
    let perlin = Perlin::new(1);
    // for i in 0..20000 {
    //     let (mut x, mut y) = (0.0, 0.0);
    //     while true {
    //         (x, y) = (rng.next_f32(), rng.next_f32());
    //         //let p = prob(x, y);
    //         let p = prob(&perlin, x, y);
    //         let e = rng.next_f32();
    //         if e < p {
    //             break;
    //         }
    //     }
    //     //let angle = rng.next_f32() * 2.0 * std::f32::consts::PI;
    //     let angle = 1.8 + rng.next_f32() * 1.0;
    //     let angle_delta = rng.next_f32() - 0.5;
    //     draw_strand(&mut pixmap, 0.005, 0.0005, x, y, angle, angle_delta, Color::WHITE)?;
    // }
    draw_trunk(3, &mut pixmap, 0.05, 0.005, 0.5, 0.8, -std::f32::consts::PI * 0.5, Color::WHITE)?;
    pixmap.save_png("out.png")?;

    Ok(())
}
