use std::io::Write;

use crate::encoders::common::to_8bit_rgb_maybe_a;
use crate::{error::MagickError, image::Image, plan::Modifiers, wm_err, wm_try};
use webp::{Encoder, WebPMemory};

pub fn encode<W: Write>(
    image: &Image,
    writer: &mut W,
    modifiers: &Modifiers,
) -> Result<(), MagickError> {
    // Convert the image to Rgb(a)8, because those are the only formats the encoder supports
    let pixels = to_8bit_rgb_maybe_a(&image.pixels);
    let encoder: Encoder = Encoder::from_image(pixels.as_ref()).unwrap();
    // imagemagick signals that the image should be lossless with quality=100
    let lossless = modifiers.quality == Some(100.0);
    // default quality is not documented, was determined experimentally
    let quality = modifiers.quality.unwrap_or(75.0) as f32;

    // Encode the image with the specified quality
    let webp: WebPMemory = encoder
        .encode_simple(lossless, quality)
        .map_err(|e| wm_err!("WebP encoding failed: {e:?}"))?;
    // TODO: `webp` crate doesn't support setting the ICC profile:
    // https://github.com/jaredforth/webp/issues/41
    wm_try!(writer.write_all(&webp));
    Ok(())
}

pub fn encode_sequence<W: Write>(
    images: &[Image],
    _: &mut W,
    _: &Modifiers,
) -> Result<(), MagickError> {
    // Use the first image to implicitly define the canvas. With modifiers we could provide
    // the right defaults to use.
    let _ = images
        .first()
        .ok_or_else(|| wm_err!("No images to encode"))?;

    // FIXME: we should use the ICC of the last (!) image, other images need conversion.
    //
    // But imagemagick handles this the same way as one may verify by grabbing a bunch of very
    // dissimilar images (e.g. <https://www.color.org/version4html.xalter>; 2025) and combining
    // them to a single sequence:
    //
    // magick -delay 100 ./Upper_Right.jpg ./Lower_Right.jpg out.webp

    // FIXME: this needs support in `webp`.
    Err(wm_err!(
        "WebP animation encoding is not yet supported, use `+adjoin` for separate frames"
    ))
}
