use serde::Serialize;
use serde::Deserialize;
use enumerations::*;

use std::path::PathBuf;

use self::filter::*;
use self::adjustment::*;
use self::compression::*;
use self::operation::*;

pub mod interface;
pub mod enumerations;

pub mod filter;
pub mod compression;
pub mod adjustment;
pub mod operation;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ImageModifier {
	BoxFilter(Box<BoxFilter>),
	Sharpen3x3(Box<Sharpen3x3>),
	GaussianBlur(Box<GaussianBlur>),
	MedianFilter(Box<MedianFilter>),
	BilateralFilter(Box<BilateralFilter>),
	SharpenGaussian(Box<SharpenGaussian>),

	Jpeg(Box<Jpeg>),
	WebP(Box<WebP>),

	Brighten(Box<Brighten>),
	Contrast(Box<Contrast>),

	Resize(Box<Resize>),
	ToLumaAlpha(Box<ToLumaAlpha>),
	ToLuma(Box<ToLuma>),
	ToRgb(Box<ToRgb>),
	ToRgba(Box<ToRgba>),
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Config {
	input: Option<Input>,
	output: Option<Output>,
	progress: Option<ProgressSettings>,

	define: Option<Vec<ImageModifier>>,
	sequence: Option<Vec<Sequence>>,
	execute: Option<Parameter>,
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
	format: SaveFormat,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ProgressSettings {
	template: String,
	chars: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sequence {
	pub id: String,
	elements: Vec<Parameter>,
}
