use serde::Serialize;
use serde::Deserialize;
use enumerations::*;

use std::path::PathBuf;
use self::filter::ImageFilter;
use self::adjustment::ImageAdjustment;
use self::compression::ImageCompression;

pub mod interface;
pub mod enumerations;

pub mod filter;
pub mod compression;
pub mod adjustment;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Config {
	input: Input,
	output: Output,
	progress: Option<ProgressSettings>,

	image: ImageSection,
	sequence: Option<Vec<Sequence>>,
	execute: Parameter,
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
pub struct ProgressSettings {
	template: String,
	chars: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ImageSection {
	format: SaveFormat,

	filter: Option<Vec<ImageFilter>>,
	adjustment: Option<Vec<ImageAdjustment>>,
	compression: Option<Vec<ImageCompression>>,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sequence {
	pub id: String,
	elements: Vec<Parameter>,
}
