use ferray::random::{default_rng_seeded, Generator};
use tiny_skia::{Color, FillRule, Paint, LineCap, PathBuilder, Pixmap, PremultipliedColorU8, Stroke, Transform};
use anyhow::{Result, bail};
use num_traits::float::FloatConst;
use noise::{NoiseFn, Perlin};

fn prob(perlin: &Perlin, x: f32, y: f32) -> f32 {
    let p1 = (perlin.get([(x * 3.0) as f64, (y * 3.0) as f64]) as f32 + 1.0) * 0.5;
    let p2 = (perlin.get([(x * 10.0) as f64, (y * 10.0) as f64]) as f32 + 1.0) * 0.5;
    let p3 = (perlin.get([(x * 20.0) as f64, (y * 20.0) as f64]) as f32 + 1.0) * 0.5;
    (p1 + p2 * 0.5 + p3 * 0.25) - 0.3
}

fn draw_trunk(levels: u32, mut rng: &mut Generator, mut pixmap: &mut Pixmap, length: f32, width: f32, x: f32, y: f32, angle: f32, color: Color) -> Result<()> {
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
    if levels == 1 {
        for i in 0..10 {
            let dang = rng.next_f32() * 2.0 * std::f32::consts::PI;
            let d = rng.next_f32() * 0.05;
            let path = PathBuilder::from_circle(x1 + d * f32::cos(dang), y1 + d * f32::sin(dang), 0.005).unwrap();
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::from_scale(pixmap.width() as f32, pixmap.width() as f32),
                None,
            );
        }
    }
    let scale_left = rng.next_f32() * 0.3 + 0.6;
    let scale_right = rng.next_f32() * 0.3 + 0.6;
    let scale_width = 0.6;
    let p = rng.next_f32();
    let ang = (rng.next_f32() - 0.5) * 2.0 * std::f32::consts::PI * 0.15;
    let ang_drift = -0.05;
    if p < 0.1 {
        draw_trunk(levels - 1, &mut rng, &mut pixmap, length * scale_left, width * scale_width, x1, y1, angle + ang_drift, color)?;
    } else {
        draw_trunk(levels - 1, &mut rng, &mut pixmap, length * scale_left, width * scale_width, x1, y1, angle - ang + ang_drift, color)?;
        draw_trunk(levels - 1, &mut rng, &mut pixmap, length * scale_right, width * scale_width, x1, y1, angle + ang + ang_drift, color)?;
    }
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
    for i in 0..20000 {
        let (mut x, mut y) = (0.0, 0.0);
        while true {
            (x, y) = (rng.next_f32(), rng.next_f32());
            //let p = prob(x, y);
            let p = prob(&perlin, x, y);
            let e = rng.next_f32();
            if e < p {
                break;
            }
        }
        //let angle = rng.next_f32() * 2.0 * std::f32::consts::PI;
        let angle = 1.8 + rng.next_f32() * 1.0;
        let angle_delta = rng.next_f32() - 0.5;
        draw_strand(&mut pixmap, 0.005, 0.0005, x, y, angle, angle_delta, Color::WHITE)?;
    }
    draw_trunk(9, &mut rng, &mut pixmap, 0.2, 0.02, 0.5, 0.9, -std::f32::consts::PI * 0.5, Color::WHITE)?;
    pixmap.save_png("out.png")?;

    Ok(())
}
