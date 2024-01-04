use serde::Serialize;
use serde::Deserialize;
use enumerations::*;

use self::filter::FilterVec;
use self::compression::CompressionVec;
use self::adjustment::AdjustmentVec;
use std::path::PathBuf;

pub mod interface;
pub mod enumerations;

pub mod filter;
pub mod compression;
pub mod adjustment;

pub type SequenceVec = Vec<Sequence>;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Config {
	input: Input,
	output: Output,
	progress_bar: Option<ProgressBar>,
	image: ImageSection,
	execute: Vec<String>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Input {
	receive: InputType,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Output {
	save: OutputType,
	path: PathBuf,
	naming: FileName,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ProgressBar {
	template: String,
	chars: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ImageSection {
	format: SaveFormat,
	filter: Option<FilterVec>,
	adjustment: Option<AdjustmentVec>,
	compression: Option<CompressionVec>,
	sequence: Option<SequenceVec>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sequence {
	pub id: String,
	instance: Vec<String>,
}
