use crate::processor::image::Modifier;
use super::*;

use std::path::Path;

impl ImageModifier {
	pub fn get(&self) -> Box<dyn Modifier> {
		match self.clone() {
			Self::BilateralFilter(m) => m,
			Self::BoxFilter(m) => m,
			Self::GaussianBlur(m) => m,
			Self::MedianFilter(m) => m,
			Self::Sharpen3x3(m) => m,
			Self::SharpenGaussian(m) => m,

			Self::Jpeg(m) => m,
			Self::WebP(m) => m,

			Self::Brighten(m) => m,
			Self::Contrast(m) => m,

			Self::Resize(m) => m,
			Self::ToLuma(m) => m,
			Self::ToLumaAlpha(m) => m,
			Self::ToRgb(m) => m,
			Self::ToRgba(m) => m,
		}
	}
}

impl Config {
	pub fn input(&self) -> Option<&Input> {
		self.input.as_ref()
	}
	pub fn output(&self) -> Option<&Output> {
		self.output.as_ref()
	}
	pub fn progress(&self) -> Option<&ProgressSettings> {
		self.progress.as_ref()
	}
	pub fn define(&self) -> Option<&Vec<ImageModifier>> {
		self.define.as_ref()
	}
	pub fn sequence(&self) -> Option<&Vec<Sequence>> {
		self.sequence.as_ref()
	}
	pub fn execute(&self) -> Option<&Parameter> {
		self.execute.as_ref()
	}
}

impl Input {
	pub fn receive(&self) -> &InputType {
		&self.receive
	}
}

impl Output {
	pub fn path(&self) -> &Path {
		&self.path
	}
}

impl ProgressSettings {
	pub fn template(&self) -> &str {
		&self.template
	}
	pub fn chars(&self) -> &str {
		&self.chars
	}
}

impl Sequence {
	pub fn id(&self) -> &str {
		&self.id
	}
	pub fn elements(&self) -> &Vec<Parameter> {
		&self.elements
	}
}
