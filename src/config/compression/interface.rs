use super::*;

use anyhow::Result;

impl Jpeg {
	pub fn quality(&self) -> Result<u32> {
		self.quality.int()
	}
}

impl WebP {
	pub fn quality(&self) -> Result<u32> {
		self.quality.int()
	}
}
