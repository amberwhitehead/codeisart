use ferray::random::{default_rng_seeded, Generator};
use tiny_skia::{Pixmap, PremultipliedColorU8};
use anyhow::{Result, bail};

fn main() -> Result<()> {
    let mut rng1 = default_rng_seeded(42);
    let scale = 1.0;
    let width = 500;
    let height = 500;
    let Some(pixmap) = Pixmap::new(
        ((width as f32) * scale) as u32,
        ((height as f32) * scale) as u32,
    ) else {
        bail!("pixmap failed");
    };

    for i in 0..1000 {
        println!("{}", rng1.next_f32());
    }
    Ok(())
}
