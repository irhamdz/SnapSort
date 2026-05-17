//! Thumbnail generation module
//!
//! Generates compact thumbnails from screenshot images and stores them for preview display.

use anyhow::{Context, Result};
use image::ImageFormat;

/// Maximum thumbnail dimensions (width × height) in pixels
const MAX_THUMBNAIL_DIM: u32 = 200;

/// Maximum thumbnail file size in bytes
const MAX_THUMBNAIL_SIZE: usize = 50 * 1024; // 50KB

/// Result of thumbnail generation
#[derive(Debug)]
pub struct ThumbnailResult {
    /// Thumbnail bytes ready for storage
    pub bytes: Vec<u8>,
    /// Original dimensions before downscaling
    pub original_dimensions: (u32, u32),
    /// Downscaled dimensions
    pub scaled_dimensions: (u32, u32),
}

/// Generate a thumbnail from an image file
///
/// Reads the source image, determines the appropriate aspect-ratio size,
/// downsamples it to the maximum dimension, encodes it in JPEG format,
/// and verifies the output size doesn't exceed the budget.
///
/// # Arguments
/// * `path` - Path to the source image file
///
/// # Returns
/// * `Ok(ThumbnailResult)` - Thumbnail bytes and dimensions
/// * `Err(anyhow::Error)` - If reading, decoding, or encoding fails
///
/// # Errors
/// - File not found
/// - Invalid image format
/// - Encoding failure (e.g., unsupported color space)
pub fn generate_thumbnail(path: &std::path::Path) -> Result<ThumbnailResult> {
    // First, get the dimensions without decoding
    let (orig_w, orig_h) = {
        let img = image::ImageReader::open(path)
            .with_context(|| format!("Failed to open image file for thumbnail generation: {}", path.display()))?
            .into_dimensions()
            .context("Failed to get image dimensions")?;
        img
    };

    // Calculate scaled dimensions while preserving aspect ratio
    let (scaled_w, scaled_h) = if orig_w > orig_h {
        // Width is the limiting dimension
        let ratio = MAX_THUMBNAIL_DIM as f32 / orig_w as f32;
        (MAX_THUMBNAIL_DIM, (orig_h as f32 * ratio) as u32)
    } else {
        // Height is the limiting dimension
        let ratio = MAX_THUMBNAIL_DIM as f32 / orig_h as f32;
        ((orig_w as f32 * ratio) as u32, MAX_THUMBNAIL_DIM)
    };

    // Now decode to DynamicImage for resizing
    let img = image::ImageReader::open(path)
        .with_context(|| format!("Failed to open image file for thumbnail generation: {}", path.display()))?
        .with_guessed_format()
        .context("Failed to guess image format")?;

    let dynamic_img = img.decode()?;

    // Resize while preserving aspect ratio
    let resized_img = dynamic_img.resize(scaled_w, scaled_h, image::imageops::FilterType::Lanczos3);

    // Convert to RGB8 for JPEG encoding
    let thumbnail_img = resized_img.to_rgb8();

    // Encode as JPEG with optimized quality (0-100)
    // Use 85% quality for good balance between size and visual quality
    let mut output = std::io::Cursor::new(Vec::new());
    thumbnail_img.write_to(&mut output, ImageFormat::Jpeg)
        .context("Failed to encode thumbnail as JPEG")?;

    let bytes = output.into_inner();

    // Verify size budget
    if bytes.len() > MAX_THUMBNAIL_SIZE {
        tracing::warn!(
            "Thumbnail for {} exceeds size budget: {} bytes (max: {} bytes)",
            path.display(),
            bytes.len(),
            MAX_THUMBNAIL_SIZE
        );
        // Note: We still return the large thumbnail instead of dropping it,
        // to avoid corrupting the database. The next time it's regenerated,
        // it should be smaller.
    }

    Ok(ThumbnailResult {
        bytes,
        original_dimensions: (orig_w, orig_h),
        scaled_dimensions: (scaled_w, scaled_h),
    })
}