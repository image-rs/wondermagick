use std::io::Write;

use image::codecs::gif::GifEncoder;
use image::DynamicImage;

use crate::{encoders::EncodeFrames, error::MagickError, image::Image, plan::Modifiers, wm_try};

pub fn encode<W: Write>(
    image: &Image,
    writer: &mut W,
    _modifiers: &Modifiers,
) -> Result<(), MagickError> {
    // speed needs to be set manually due to https://github.com/image-rs/image/issues/2506
    let encoder = GifEncoder::new_with_speed(writer, 10);
    wm_try!(image.pixels.write_with_encoder(encoder));
    Ok(())
}

pub fn encode_sequence<W: Write>(
    images: &[Image],
    writer: &mut W,
    _modifiers: &Modifiers,
) -> Result<(), MagickError> {
    let mut encoder = GifEncoder::new_with_speed(writer, 10);

    for image in images {
        wm_try!(<dyn EncodeFrames>::write_dynimage(
            &mut encoder,
            &image.pixels
        ));
    }

    Ok(())
}

impl<W: Write> EncodeFrames for GifEncoder<W> {
    fn encode(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        color: image::ExtendedColorType,
    ) -> Result<(), MagickError> {
        wm_try!(self.encode(data, width, height, color));
        Ok(())
    }

    fn make_compatible(&self, img: &DynamicImage) -> Option<DynamicImage> {
        match img {
            DynamicImage::ImageRgb8(_)
            | DynamicImage::ImageRgba8(_)
            | DynamicImage::ImageLuma8(_) => None,
            DynamicImage::ImageLumaA8(_)
            | DynamicImage::ImageLuma16(_)
            | DynamicImage::ImageLumaA16(_) => Some(DynamicImage::ImageLuma8(img.to_luma8())),
            DynamicImage::ImageRgb16(_) | DynamicImage::ImageRgb32F(_) => {
                Some(DynamicImage::ImageRgb8(img.to_rgb8()))
            }
            DynamicImage::ImageRgba16(_) | DynamicImage::ImageRgba32F(_) => {
                Some(DynamicImage::ImageRgba8(img.to_rgba8()))
            }
            // Whatever extension `image` might make, handle it as RGBA8
            _ => Some(DynamicImage::ImageRgba8(img.to_rgba8())),
        }
    }
}
