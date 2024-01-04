use crate::processor::image::filter::Filter;
use super::*;

use anyhow::Result;
use anyhow::anyhow;
use resize::Type;

impl ImageFilter {
	pub fn get(&self) -> Box<dyn Filter> {
		match self.clone() {
			Self::BilateralFilter(f) => f,
			Self::BoxFilter(f) => f,
			Self::GaussianBlur(f) => f,
			Self::MedianFilter(f) => f,
			Self::Sharpen3x3(f) => f,
			Self::Resize(f) => f,
			Self::SharpenGaussian(f) => f,
		}
	}
}

impl BilateralFilter {
	pub fn window_size(&self) -> Result<u32> {
		self.window_size.int()
	}
	pub fn sigma_color(&self) -> Result<f32> {
		self.sigma_color.float()
	}
	pub fn sigma_spatial(&self) -> Result<f32> {
		self.sigma_spatial.float()
	}
}
impl BoxFilter {
	pub fn x_radius(&self) -> Result<u32> {
		self.x_radius.int()
	}
	pub fn y_radius(&self) -> Result<u32> {
		self.y_radius.int()
	}
}

impl GaussianBlur {
	pub fn sigma(&self) -> Result<f32> {
		self.sigma.float()
	}
}
impl SharpenGaussian {
	pub fn sigma(&self) -> Result<f32> {
		self.sigma.float()
	}
	pub fn amount(&self) -> Result<f32> {
		self.amount.float()
	}
}

impl MedianFilter {
	pub fn x_radius(&self) -> Result<u32> {
		self.x_radius.int()
	}
	pub fn y_radius(&self) -> Result<u32> {
		self.y_radius.int()
	}
}

impl Resize {
	pub fn width(&self) -> Result<u32> {
		if let Some(width) = self.width.as_ref() { width.int() } else { Err(anyhow!("width error")) }
	}
	pub fn height(&self) -> Result<u32> {
		if let Some(height) = self.height.as_ref() {
			height.int()
		} else {
			Err(anyhow!("height error "))
		}
	}
	pub fn filter(&self) -> Type {
		match self.filter {
			ResizeFilter::Point => Type::Point,
			ResizeFilter::Triangle => Type::Triangle,
			ResizeFilter::Catrom => Type::Catrom,
			ResizeFilter::Mitchell => Type::Mitchell,
			ResizeFilter::BSpline => Type::BSpline,
			ResizeFilter::Gaussian => Type::Gaussian,
			ResizeFilter::Lanczos3 => Type::Lanczos3,
		}
	}
}
