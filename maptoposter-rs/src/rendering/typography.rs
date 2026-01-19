use std::path::Path;

use fontdue::{Font, FontSettings};
use tiny_skia::Pixmap;

use crate::error::{AppError, Result};
use crate::themes::loader::parse_hex_color;

/// Font collection for text rendering
pub struct FontSet {
    pub bold: Font,
    pub regular: Font,
    pub light: Font,
}

impl FontSet {
    /// Load fonts from a directory
    pub fn load(fonts_dir: &Path) -> Result<Self> {
        let bold = load_font(fonts_dir.join("Roboto-Bold.ttf"))?;
        let regular = load_font(fonts_dir.join("Roboto-Regular.ttf"))?;
        let light = load_font(fonts_dir.join("Roboto-Light.ttf"))?;

        Ok(Self {
            bold,
            regular,
            light,
        })
    }
}

fn load_font(path: impl AsRef<Path>) -> Result<Font> {
    let data = std::fs::read(path.as_ref()).map_err(|e| {
        AppError::Rendering(format!("Failed to read font {:?}: {}", path.as_ref(), e))
    })?;

    Font::from_bytes(data, FontSettings::default()).map_err(|e| {
        AppError::Rendering(format!("Failed to load font {:?}: {}", path.as_ref(), e))
    })
}

/// Render text onto a pixmap
pub fn render_text(
    pixmap: &mut Pixmap,
    text: &str,
    font: &Font,
    size: f32,
    hex_color: &str,
    x: f32,
    y: f32,
    centered: bool,
    letter_spacing: f32,
) {
    let (r, g, b) = match parse_hex_color(hex_color) {
        Some(c) => c,
        None => return,
    };

    // Calculate total width for centering
    let mut total_width = 0.0;
    let mut char_metrics: Vec<(fontdue::Metrics, Vec<u8>, char)> = Vec::new();

    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, size);
        total_width += metrics.advance_width + letter_spacing;
        char_metrics.push((metrics, bitmap, c));
    }

    // Adjust x for centering
    let start_x = if centered {
        x - total_width / 2.0
    } else {
        x
    };

    // Render each character
    let mut cursor_x = start_x;
    let width = pixmap.width() as usize;
    let height = pixmap.height() as usize;
    let pixels = pixmap.pixels_mut();

    for (metrics, bitmap, _c) in char_metrics {
        let glyph_x = cursor_x + metrics.xmin as f32;
        let glyph_y = y - metrics.ymin as f32 - metrics.height as f32;

        // Render the glyph bitmap
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let alpha = bitmap[gy * metrics.width + gx];
                if alpha == 0 {
                    continue;
                }

                let px = (glyph_x + gx as f32) as usize;
                let py = (glyph_y + gy as f32) as usize;

                if px < width && py < height {
                    let idx = py * width + px;
                    blend_text_pixel(&mut pixels[idx], r, g, b, alpha);
                }
            }
        }

        cursor_x += metrics.advance_width + letter_spacing;
    }
}

/// Blend text pixel onto existing pixel
fn blend_text_pixel(pixel: &mut tiny_skia::PremultipliedColorU8, r: u8, g: u8, b: u8, alpha: u8) {
    if alpha == 0 {
        return;
    }

    let a = alpha as f32 / 255.0;
    let inv_a = 1.0 - a;

    let existing_a = pixel.alpha();
    if existing_a == 0 {
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

    let new_r = (r as f32 * a + existing_r * inv_a).min(255.0) as u8;
    let new_g = (g as f32 * a + existing_g * inv_a).min(255.0) as u8;
    let new_b = (b as f32 * a + existing_b * inv_a).min(255.0) as u8;
    let new_a = ((alpha as f32 + existing_a as f32 * inv_a)).min(255.0) as u8;

    *pixel = tiny_skia::PremultipliedColorU8::from_rgba(
        (new_r as f32 * new_a as f32 / 255.0) as u8,
        (new_g as f32 * new_a as f32 / 255.0) as u8,
        (new_b as f32 * new_a as f32 / 255.0) as u8,
        new_a,
    )
    .unwrap();
}

/// Render a decorative line
pub fn render_line(pixmap: &mut Pixmap, hex_color: &str, y: f32, width_ratio: f32, thickness: f32) {
    let (r, g, b) = match parse_hex_color(hex_color) {
        Some(c) => c,
        None => return,
    };

    let pix_width = pixmap.width();
    let pix_height = pixmap.height();

    let line_width = (pix_width as f32 * width_ratio) as u32;
    let start_x = (pix_width - line_width) / 2;
    let end_x = start_x + line_width;

    let start_y = y as u32;
    let end_y = (start_y + thickness as u32).min(pix_height);

    let pixels = pixmap.pixels_mut();

    for py in start_y..end_y {
        for px in start_x..end_x {
            let idx = (py * pix_width + px) as usize;
            if idx < pixels.len() {
                pixels[idx] =
                    tiny_skia::PremultipliedColorU8::from_rgba(r, g, b, 255).unwrap();
            }
        }
    }
}

/// Render all poster typography (city, country, coordinates, attribution)
pub fn render_poster_typography(
    pixmap: &mut Pixmap,
    fonts: &FontSet,
    city: &str,
    country: &str,
    coordinates: &str,
    text_color: &str,
) {
    let width = pixmap.width() as f32;
    let height = pixmap.height() as f32;
    let center_x = width / 2.0;

    // City name (with letter spacing) - y=0.14
    let city_y = height * 0.86;
    let city_size = height * 0.04; // Larger font for city
    render_text(
        pixmap,
        &city.to_uppercase(),
        &fonts.bold,
        city_size,
        text_color,
        center_x,
        city_y,
        true,
        city_size * 0.3, // Letter spacing
    );

    // Decorative line - y=0.125
    let line_y = height * 0.875;
    render_line(pixmap, text_color, line_y, 0.2, 2.0);

    // Country name - y=0.10
    let country_y = height * 0.90;
    let country_size = height * 0.015;
    render_text(
        pixmap,
        &country.to_uppercase(),
        &fonts.regular,
        country_size,
        text_color,
        center_x,
        country_y,
        true,
        country_size * 0.2,
    );

    // Coordinates - y=0.07
    let coords_y = height * 0.93;
    let coords_size = height * 0.01;
    render_text(
        pixmap,
        coordinates,
        &fonts.light,
        coords_size,
        text_color,
        center_x,
        coords_y,
        true,
        0.0,
    );

    // Attribution - bottom right
    let attr_y = height * 0.98;
    let attr_size = height * 0.006;
    let attr_x = width * 0.98;
    render_text(
        pixmap,
        "Map data © OpenStreetMap",
        &fonts.light,
        attr_size,
        text_color,
        attr_x,
        attr_y,
        false,
        0.0,
    );
}
