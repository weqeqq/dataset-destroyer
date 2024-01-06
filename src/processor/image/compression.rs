use super::Image;
use crate::config::compression::*;

use image::DynamicImage;
use image::codecs as codecs;
use codecs::jpeg::JpegDecoder;
use codecs::jpeg::JpegEncoder;
use codecs::webp::WebPDecoder;
use codecs::webp::WebPEncoder;
use codecs::webp::WebPQuality;
use std::io::Cursor;

use anyhow::Result;
use super::Modifier;

impl Modifier for Jpeg {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let quality = self.quality()? as u8;

		let mut buffer = Vec::new();
		let mut cursor = Cursor::new(&mut buffer);

		let encoder = JpegEncoder::new_with_quality(&mut cursor, quality);
		image.source_mut().write_with_encoder(encoder)?;

		cursor.set_position(0);
		let decoder = JpegDecoder::new(&mut cursor)?;
		*image.source_mut() = DynamicImage::from_decoder(decoder)?;

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Modifier for WebP {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let quality = self.quality()? as u8;

		let mut buffer = Vec::new();
		let mut cursor = Cursor::new(&mut buffer);

		let encoder = WebPEncoder::new_with_quality(&mut cursor, WebPQuality::lossy(quality));
		image.source_mut().write_with_encoder(encoder)?;

		cursor.set_position(0);
		let decoder = WebPDecoder::new(&mut cursor)?;
		*image.source_mut() = DynamicImage::from_decoder(decoder)?;

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}
