use super::*;
use super::enumerations::SaveFormat;
use std::path::Path;
use anyhow::Result;

impl Config {
	pub fn input(&self) -> &Input {
		&self.input
	}
	pub fn output(&self) -> &Output {
		&self.output
	}
	pub fn progress_bar(&self) -> Option<&ProgressBarConf> {
		self.progress_bar.as_ref()
	}
	pub fn image_section(&self) -> &ImageSection {
		&self.image
	}
	pub fn sequence(&self) -> Option<&SequenceVec> {
		self.sequence.as_ref()
	}
	pub fn execute(&self) -> Result<Vec<&str>> {
		self.execute.id_seq()
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

impl ProgressBarConf {
	pub fn template(&self) -> &str {
		&self.template
	}
	pub fn chars(&self) -> &str {
		&self.chars
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

impl Sequence {
	pub fn id(&self) -> &str {
		&self.id
	}
	pub fn elements(&self) -> &Vec<Parameter> {
		&self.elements
	}
}