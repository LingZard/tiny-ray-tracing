use std::io::Write;

use super::interval::Interval;
use super::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> std::io::Result<()> {
    let pixel_color = linear_to_gamma(pixel_color);
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let interval = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * interval.clamp(r)) as u32;
    let gbyte = (256.0 * interval.clamp(g)) as u32;
    let bbyte = (256.0 * interval.clamp(b)) as u32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}

pub fn linear_to_gamma(color: &Color) -> Color {
    Color::new(
        color.x().powf(1.0 / 2.2),
        color.y().powf(1.0 / 2.2),
        color.z().powf(1.0 / 2.2),
    )
}
