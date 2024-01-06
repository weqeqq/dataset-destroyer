use super::*;

use anyhow::Result;

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
