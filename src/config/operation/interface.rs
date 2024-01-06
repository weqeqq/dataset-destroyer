use super::*;
use resize::Type;
use anyhow::Result;
use anyhow::anyhow;

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
