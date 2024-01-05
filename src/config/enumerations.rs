use serde::Serialize;
use serde::Deserialize;

#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum ResizeFilter {
	Point,
	Catrom,
	BSpline,
	Mitchell,
	Lanczos3,
	Gaussian,
	Triangle,
}

#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum SaveFormat {
	Png,
	Jpeg,
	Webp,
	Original,
}

#[derive(PartialEq)]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Parameter {
	Id(String),
	RandId(Vec<String>),
	RandMulId(Vec<String>, usize),

	Percent(u32), // todo
	RandPercent(), // todo
	RandPercentFrom(Vec<u32>), // todo

	Float(f32),
	RandFloat(f32, f32), 
	RandFloatFrom(Vec<f32>), 

	Int(u32),
	RandInt(usize, usize), 
	RandIntFrom(Vec<u32>),

	RandResizeFilter, // todo
	RandResizeFilterFrom(Vec<ResizeFilter>), // todo
	RandResizeFilterWithout(Vec<ResizeFilter>), // todo

	RandFormat, // todo
	RandFormatFrom(Vec<SaveFormat>), // todo
	RandFormatWithout(Vec<SaveFormat>), // todo
}

pub enum IdType {
	Filter,
	Compression,
	Adjustment,
	Sequence,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum InputType {
	Path(String),
	PathArr(Vec<String>),
	PathFile, // todo
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum OutputType {
	Each,
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum FileName {
	Original,
	Pattern, // todo
}
