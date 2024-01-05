use crate::processor::image::ImageModifier;
use super::*;

use anyhow::Result;

impl ImageCompression {
	pub fn get(&self) -> Box<dyn ImageModifier> {
		match self.clone() {
			Self::Jpeg(m) => m,
			Self::WebP(m) => m,
		}
	}
}

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
