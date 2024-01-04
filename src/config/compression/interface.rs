use crate::processor::image::compression::Compression;
use super::*;
use anyhow::Result;

impl ImageCompression {
	pub fn get(&self) -> Box<dyn Compression> {
		match self.clone() {
			Self::Jpeg(c) => c,
			Self::WebP(c) => c,
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
