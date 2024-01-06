use super::*;

use anyhow::Result;

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
