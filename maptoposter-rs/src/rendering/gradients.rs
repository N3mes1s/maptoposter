use tiny_skia::Pixmap;

use crate::themes::loader::parse_hex_color;

/// Location for gradient fade
#[derive(Debug, Clone, Copy)]
pub enum GradientLocation {
    Top,
    Bottom,
}

/// Apply a gradient fade overlay to the pixmap
pub fn apply_gradient_fade(pixmap: &mut Pixmap, hex_color: &str, location: GradientLocation) {
    let (r, g, b) = match parse_hex_color(hex_color) {
        Some(c) => c,
        None => return,
    };

    let height = pixmap.height();
    let width = pixmap.width();

    // Gradient covers 25% of the image height
    let gradient_height = (height as f32 * 0.25) as u32;

    let pixels = pixmap.pixels_mut();

    match location {
        GradientLocation::Bottom => {
            // Fade from bottom edge (y=0 in image coords, but we flip)
            for y in 0..gradient_height {
                // Alpha: 255 at bottom edge, 0 at gradient end
                let alpha = ((1.0 - y as f32 / gradient_height as f32) * 255.0) as u8;

                for x in 0..width {
                    let idx = (y * width + x) as usize;
                    blend_pixel(&mut pixels[idx], r, g, b, alpha);
                }
            }
        }
        GradientLocation::Top => {
            // Fade from top edge
            let start_y = height - gradient_height;
            for y in 0..gradient_height {
                // Alpha: 0 at gradient start, 255 at top edge
                let alpha = (y as f32 / gradient_height as f32 * 255.0) as u8;

                for x in 0..width {
                    let idx = ((start_y + y) * width + x) as usize;
                    blend_pixel(&mut pixels[idx], r, g, b, alpha);
                }
            }
        }
    }
}

/// Blend a color onto an existing pixel using alpha compositing
fn blend_pixel(pixel: &mut tiny_skia::PremultipliedColorU8, r: u8, g: u8, b: u8, alpha: u8) {
    if alpha == 0 {
        return;
    }

    let a = alpha as f32 / 255.0;
    let inv_a = 1.0 - a;

    // Demultiply the existing pixel
    let existing_a = pixel.alpha();
    if existing_a == 0 {
        // Fully transparent, just set the new color
        *pixel = tiny_skia::PremultipliedColorU8::from_rgba(
            (r as f32 * a) as u8,
            (g as f32 * a) as u8,
            (b as f32 * a) as u8,
            alpha,
        )
        .unwrap();
        return;
    }

    let existing_r = pixel.red() as f32 / existing_a as f32 * 255.0;
    let existing_g = pixel.green() as f32 / existing_a as f32 * 255.0;
    let existing_b = pixel.blue() as f32 / existing_a as f32 * 255.0;

    // Blend
    let new_r = (r as f32 * a + existing_r * inv_a).min(255.0) as u8;
    let new_g = (g as f32 * a + existing_g * inv_a).min(255.0) as u8;
    let new_b = (b as f32 * a + existing_b * inv_a).min(255.0) as u8;
    let new_a = ((alpha as f32 + existing_a as f32 * inv_a)).min(255.0) as u8;

    // Pre-multiply and set
    *pixel = tiny_skia::PremultipliedColorU8::from_rgba(
        (new_r as f32 * new_a as f32 / 255.0) as u8,
        (new_g as f32 * new_a as f32 / 255.0) as u8,
        (new_b as f32 * new_a as f32 / 255.0) as u8,
        new_a,
    )
    .unwrap();
}

/// Apply both top and bottom gradient fades
pub fn apply_gradient_fades(pixmap: &mut Pixmap, hex_color: &str) {
    apply_gradient_fade(pixmap, hex_color, GradientLocation::Bottom);
    apply_gradient_fade(pixmap, hex_color, GradientLocation::Top);
}
