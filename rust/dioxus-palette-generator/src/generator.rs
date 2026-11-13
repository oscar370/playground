use crate::errors::AppError;
use anyhow::Context;
use dioxus::html::bytes::Bytes;
use image::{imageops::FilterType, ImageReader};
use material_colors::{
    color::Rgb,
    image::extract_color,
    theme::{Theme, ThemeBuilder},
};
use std::io::Cursor;

pub fn build_theme_with_bytes(bytes: Bytes) -> Result<Theme, AppError> {
    let image = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .context("failed to guess image format")?
        .decode()
        .context("failed to decode the image")?;

    let pixels: Vec<Rgb> = image
        .resize(128, 128, FilterType::Lanczos3)
        .into_rgb8()
        .pixels()
        .map(|p| {
            let [r, g, b] = p.0;
            Rgb::new(r, g, b)
        })
        .collect();

    Ok(ThemeBuilder::with_source(extract_color(&pixels)).build())
}
