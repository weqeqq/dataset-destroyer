use crate::processor::image::adjustment::Adjustment;
use super::*;
use anyhow::Result;

impl ImageAdjustment{
	pub fn get(&self) -> Box<dyn Adjustment>{
		match self.clone() {
			Self::Brighten(a) => a,
			Self::Contrast(a) => a,
		}
	}
}

impl Brighten {
	pub fn value(&self) -> Result<u32> {
		self.value.int()
	}
}

impl Contrast {
	pub fn contrast(&self) -> Result<f32> {
		self.contrast.float()
	}
}
