#[cfg(feature = "avif")]
pub mod avif;
pub mod common;
#[cfg(feature = "gif")]
pub mod gif;
#[cfg(feature = "jpeg")]
pub mod jpeg;
#[cfg(feature = "png")]
pub mod png;
#[cfg(feature = "tiff")]
pub mod tiff;
#[cfg(feature = "webp")]
pub mod webp;

/// Trait for encoders that support encoding multiple frames (images) into a single file.
pub trait EncodeFrames {
    fn encode(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        color: image::ExtendedColorType,
    ) -> Result<(), crate::error::MagickError>;

    fn make_compatible(&self, img: &image::DynamicImage) -> Option<image::DynamicImage>;
}

impl dyn EncodeFrames + '_ {
    fn write_dynimage(
        &mut self,
        img: &image::DynamicImage,
    ) -> Result<(), crate::error::MagickError> {
        use image::GenericImageView as _;
        let converted = self.make_compatible(img);
        let img = converted.as_ref().unwrap_or(img);

        let data = img.as_bytes();
        let (width, height) = img.dimensions();
        let color = img.color();

        self.encode(&data, width, height, color.into())
    }
}
