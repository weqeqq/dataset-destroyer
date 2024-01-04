use super::*;
use super::enumerations::SaveFormat;
use std::path::Path;

impl Config {
	pub fn input(&self) -> &Input {
		&self.input
	}
	pub fn output(&self) -> &Output {
		&self.output
	}
	pub fn progress_bar(&self) -> Option<&ProgressBar> {
		self.progress_bar.as_ref()
	}
	pub fn image_section(&self) -> &ImageSection {
		&self.image
	}
	pub fn execute(&self) -> &Vec<String> {
		&self.execute
	}
}

impl Input {
	pub fn receive(&self) -> &InputType {
		&self.receive
	}
}

impl Output {
	pub fn path(&self) -> &Path {
		self.path.as_ref()
	}
}

impl ProgressBar {
	pub fn template(&self) -> &str {
		self.template.as_ref()
	}
	pub fn chars(&self) -> &str {
		self.chars.as_ref()
	}
}

impl ImageSection {
	pub fn format(&self) -> SaveFormat {
		self.format
	}
	pub fn filter(&self) -> Option<&FilterVec> {
		self.filter.as_ref()
	}
	pub fn compression(&self) -> Option<&CompressionVec> {
		self.compression.as_ref()
	}
	pub fn adjustment(&self) -> Option<&AdjustmentVec> {
		self.adjustment.as_ref()
	}
}
