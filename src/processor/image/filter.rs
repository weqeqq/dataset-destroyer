use crate::config::filter::*;
use super::Image;

use anyhow::Result;
use anyhow::anyhow;

use imageproc::filter::*;
use image::DynamicImage::*;
use image::GrayImage;
use image::RgbImage;
use image::RgbaImage;

use rgb::FromSlice;
use num_traits::AsPrimitive;

pub trait Filter {
	fn apply(&self, image: &mut Image) -> Result<()>;
	fn id(&self) -> &str;
}

impl Filter for GaussianBlur {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let sigma = self.sigma()?;

		*image.source_mut() = match image.source() {
			ImageLuma8(image) => ImageLuma8(gaussian_blur_f32(image, sigma)),
			ImageLuma16(image) => ImageLuma16(gaussian_blur_f32(image, sigma)),
			ImageLumaA8(image) => ImageLumaA8(gaussian_blur_f32(image, sigma)),
			ImageLumaA16(image) => ImageLumaA16(gaussian_blur_f32(image, sigma)),
			ImageRgb8(image) => ImageRgb8(gaussian_blur_f32(image, sigma)),
			ImageRgb16(image) => ImageRgb16(gaussian_blur_f32(image, sigma)),
			ImageRgb32F(image) => ImageRgb32F(gaussian_blur_f32(image, sigma)),
			ImageRgba8(image) => ImageRgba8(gaussian_blur_f32(image, sigma)),
			ImageRgba16(image) => ImageRgba16(gaussian_blur_f32(image, sigma)),
			ImageRgba32F(image) => ImageRgba32F(gaussian_blur_f32(image, sigma)),
			_ => unimplemented!(),
		};

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Filter for BilateralFilter {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let window_size = self.window_size()?;
		let sigma_color = self.sigma_color()?;
		let sigma_spatial = self.sigma_spatial()?;

		let channels = image.separate_channels()?;
		let mut filtered_channels = Vec::new();

		for channel in channels {
			let filtered = bilateral_filter(&channel, window_size, sigma_color, sigma_spatial);
			filtered_channels.push(filtered);
		}

		let combined = image.combine_channels(filtered_channels);
		*image.source_mut() = combined;

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Filter for BoxFilter {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let x_radius = self.x_radius()?;
		let y_radius = self.y_radius()?;

		let channels = image.separate_channels()?;
		let mut filtered_channels = Vec::new();

		for channel in channels {
			let filtered = box_filter(&channel, x_radius, y_radius);
			filtered_channels.push(filtered);
		}

		let combined = image.combine_channels(filtered_channels);
		*image.source_mut() = combined;

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Filter for Sharpen3x3 {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let channels = image.separate_channels()?;
		let mut filtered_channels = Vec::new();

		for channel in channels {
			let filtered = sharpen3x3(&channel);
			filtered_channels.push(filtered);
		}

		let combined = image.combine_channels(filtered_channels);
		*image.source_mut() = combined;

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Filter for SharpenGaussian {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let sigma = self.sigma()?;
		let amount = self.amount()?;

		let channels = image.separate_channels()?;
		let mut filtered_channels = Vec::new();

		for channel in channels {
			let filtered = sharpen_gaussian(&channel, sigma, amount);
			filtered_channels.push(filtered);
		}

		let combined = image.combine_channels(filtered_channels);
		*image.source_mut() = combined;

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Filter for MedianFilter {
	fn apply(&self, image: &mut Image) -> Result<()> {
		let x_radius = self.x_radius()?;
		let y_radius = self.y_radius()?;

		*image.source_mut() = match image.source() {
			ImageLuma8(image) => ImageLuma8(median_filter(image, x_radius, y_radius)),
			ImageLumaA8(image) => ImageLumaA8(median_filter(image, x_radius, y_radius)),
			ImageRgb8(image) => ImageRgb8(median_filter(image, x_radius, y_radius)),
			ImageRgba8(image) => ImageRgba8(median_filter(image, x_radius, y_radius)),
			_ => unimplemented!(),
		};

		Ok(())
	}
	fn id(&self) -> &str {
		self.id.as_ref()
	}
}

impl Filter for Resize {
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
