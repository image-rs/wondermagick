use std::io::Write;

use image::codecs::avif::AvifEncoder;
use image::ImageEncoder;

use crate::{error::MagickError, image::Image, plan::Modifiers, wm_err, wm_try};

pub fn encode<W: Write>(
    image: &Image,
    writer: &mut W,
    modifiers: &Modifiers,
) -> Result<(), MagickError> {
    // TODO: quality conversion might not be bug-compatible with imagemagick
    let quality = modifiers.quality.map(|q| q as u8).unwrap_or(50);
    let mut encoder = AvifEncoder::new_with_speed_quality(writer, 4, quality);
    if let Some(icc) = image.icc.clone() {
        let _ = encoder.set_icc_profile(icc); // ignore UnsupportedError
    };
    // ravif already discards alpha channel automatically if all pixels are opaque,
    // so no need to explicitly convert on our end
    wm_try!(image.pixels.write_with_encoder(encoder));
    Ok(())
}

pub fn encode_sequence<W: Write>(
    _: &[Image],
    writer: &mut W,
    modifiers: &Modifiers,
) -> Result<(), MagickError> {
    let quality = modifiers.quality.map(|q| q as u8).unwrap_or(50);
    let _encoder = AvifEncoder::new_with_speed_quality(writer, 4, quality);

    // ICC is stored for each individual frame. Imagemagick also knows to extract them again but
    // libavif seems to disagree with the meta information for storing them? Not sure why but it
    // reports just one frame where `magick identify` reports them all but with missing delay /
    // duration information. Profiles are verifiably stored in full when extracting the frames into
    // separate files with `magick joined.avif[0] frame-0.png` etc.
    Err(wm_err!(
        "Avif animation encoding is not yet supported, use `+adjoin` for separate frames"
    ))
}
