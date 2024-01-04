use serde::Serialize;
use serde::Deserialize;
use super::enumerations::*;

mod interface;
pub type FilterVec = Vec<ImageFilter>;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ImageFilter {
	Resize(Box<Resize>),
	BoxFilter(Box<BoxFilter>),
	Sharpen3x3(Box<Sharpen3x3>),
	GaussianBlur(Box<GaussianBlur>),
	MedianFilter(Box<MedianFilter>),
	BilateralFilter(Box<BilateralFilter>),
	SharpenGaussian(Box<SharpenGaussian>),
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BilateralFilter {
	pub id: String,
	window_size: Parameter,
	sigma_color: Parameter,
	sigma_spatial: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BoxFilter {
	pub id: String,
	x_radius: Parameter,
	y_radius: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct GaussianBlur {
	pub id: String,
	sigma: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sharpen3x3 {
	pub id: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SharpenGaussian {
	pub id: String,
	sigma: Parameter,
	amount: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct MedianFilter {
	pub id: String,
	x_radius: Parameter,
	y_radius: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Resize {
	pub id: String,
	width: Option<Parameter>,
	height: Option<Parameter>,
	filter: ResizeFilter,
}
