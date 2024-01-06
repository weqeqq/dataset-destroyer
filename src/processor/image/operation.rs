use crate::config::operation::*;
use super::Image;
use super::Modifier;

use anyhow::Result;
use anyhow::anyhow;

use image::DynamicImage::*;
use image::GrayImage;
use image::RgbImage;
use image::RgbaImage;

use rgb::FromSlice;
use num_traits::AsPrimitive;

impl Modifier for Resize {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let w = image.width() as usize;
		let h = image.height() as usize;

		let nw = self.width()? as usize;
		let nh = self.height()? as usize;

		let color = image.color();
		let channel_count = color.channel_count() as usize;

		let filter = self.filter();
		let mut dst = vec![0u8; (nw * nh * channel_count).as_()];

		let gray = image
			.source()
			.as_luma8()
			.and_then(|luma| Some(luma.as_gray()));

		let rgb = image
			.source()
			.as_rgb8()
			.and_then(|rgb| Some(rgb.as_rgb()));

		let rgba = image
			.source()
			.as_rgba8()
			.and_then(|rgba| Some(rgba.as_rgba()));

		if let Some(gray) = gray {
			let mut resizer = resize::new(w, h, nw, nh, resize::Pixel::Gray8, filter)?;
			resizer.resize(gray, dst.as_gray_mut())?;

			let mut buffer = GrayImage::new(nw.as_(), nh.as_());
			buffer
				.iter_mut()
				.zip(dst.iter())
				.for_each(|(buf, dst)| {
					*buf = *dst;
				});

			*image.source_mut() = ImageLuma8(buffer);

			return Ok(());
		}

		if let Some(rgb) = rgb {
			let mut resizer = resize::new(w, h, nw, nh, resize::Pixel::RGB8, filter)?;
			resizer.resize(rgb, dst.as_rgb_mut())?;

			let mut buffer = RgbImage::new(nw.as_(), nh.as_());
			buffer
				.iter_mut()
				.zip(dst.iter())
				.for_each(|(buf, dst)| {
					*buf = *dst;
				});

			*image.source_mut() = ImageRgb8(buffer);

			return Ok(());
		}

		if let Some(rgba) = rgba {
			let mut resizer = resize::new(w, h, nw, nh, resize::Pixel::RGBA8, filter)?;
			resizer.resize(rgba, dst.as_rgba_mut())?;

			let mut buffer = RgbaImage::new(nw.as_(), nh.as_());
			buffer
				.iter_mut()
				.zip(dst.iter())
				.for_each(|(buf, dst)| {
					*buf = *dst;
				});

			*image.source_mut() = ImageRgba8(buffer);
			return Ok(());
		}

		Err(anyhow!("unsupported color"))
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Modifier for ToLumaAlpha {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let luma_alpha = image.source().to_luma_alpha8();
		*image.source_mut() = ImageLumaA8(luma_alpha);

		Ok(())
	}
	fn id(&self) -> &str {
		&self.id
	}
}

impl Modifier for ToLuma {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let luma = image.source().to_luma8();
		*image.source_mut() = ImageLuma8(luma);

		Ok(())
	}
	fn id(&self) -> &str {
		&self.id
	}
}

impl Modifier for ToRgba {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let rgba = image.source().to_rgba8();
		*image.source_mut() = ImageRgba8(rgba);

		Ok(())
	}
	fn id(&self) -> &str {
		&self.id
	}
}

impl Modifier for ToRgb {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let rgb = image.source().to_rgb8();
		*image.source_mut() = ImageRgb8(rgb);

		Ok(())
	}
	fn id(&self) -> &str {
		&self.id
	}
}
