use crate::config::filter::*;
use super::Image;
use super::Modifier;

use anyhow::Result;

use imageproc::filter::*;
use image::DynamicImage::*;

impl Modifier for GaussianBlur {
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

impl Modifier for BilateralFilter {
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

impl Modifier for BoxFilter {
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

impl Modifier for Sharpen3x3 {
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

impl Modifier for SharpenGaussian {
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

impl Modifier for MedianFilter {
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
