use std::io::{Cursor, Write};

use image::codecs::tiff::TiffEncoder;

use crate::encoders::common::{optimize_pixel_format_and_precision, write_icc_and_exif};
use crate::encoders::EncodeFrames;
use crate::plan::Modifiers;
use crate::wm_err;
use crate::{error::MagickError, image::Image, wm_try};

pub fn encode<W: Write>(
    image: &Image,
    writer: &mut W,
    _modifiers: &Modifiers,
) -> Result<(), MagickError> {
    // We need Seek for TiffEncoder, so write to a buffer first
    let mut data = Cursor::new(vec![]);
    let mut encoder = TiffEncoder::new(&mut data);

    write_icc_and_exif(&mut encoder, image);
    let pixels_to_write = optimize_pixel_format_and_precision(&image.pixels);
    wm_try!(pixels_to_write.write_with_encoder(encoder));
    wm_try!(writer.write_all(&data.into_inner()));

    Ok(())
}

pub fn encode_sequence<W: Write>(
    images: &[Image],
    writer: &mut W,
    _modifiers: &Modifiers,
) -> Result<(), MagickError> {
    let mut data = Cursor::new(vec![]);
    let mut encoder = TiffEncoder::new(&mut data);

    for image in images {
        write_icc_and_exif(&mut encoder, image);
        let pixels_to_write = optimize_pixel_format_and_precision(&image.pixels);

        wm_try!(<dyn EncodeFrames>::write_dynimage(
            &mut encoder,
            &pixels_to_write
        ));
    }

    wm_try!(writer.write_all(&data.into_inner()));
    Ok(())
}

impl<W: Write + std::io::Seek> EncodeFrames for TiffEncoder<W> {
    fn encode(
        &mut self,
        _: &[u8],
        _: u32,
        _: u32,
        _: image::ExtendedColorType,
    ) -> Result<(), crate::error::MagickError> {
        Err(wm_err!("TIFF multi-frame encoding is not supported yet"))
    }

    fn make_compatible(&self, _: &image::DynamicImage) -> Option<image::DynamicImage> {
        None
    }
}
