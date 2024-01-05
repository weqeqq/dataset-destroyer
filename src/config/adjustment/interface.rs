use crate::processor::image::ImageModifier;
use super::*;

use anyhow::Result;

impl ImageAdjustment {
	pub fn get(&self) -> Box<dyn ImageModifier> {
		match self.clone() {
			Self::Brighten(m) => m,
			Self::Contrast(m) => m,
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
