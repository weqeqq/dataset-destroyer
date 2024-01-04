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

	Percentage(u32), // todo
	RandomPercentage(u32, u32), // todo

	Float(f32),
	RandFloat(f32, f32), // todo
	RandFloatFrom(Vec<f32>), // todo
	RandFloatWithStep(f32, f32, f32), // todo

	Int(u32),
	RandInt(usize, usize), // todo
	RandIntFrom(Vec<u32>), // todo
	RandIntWithStep(usize, usize, usize), // todo

	RandResizeFilter, // todo
	RandResizeFilterFrom(Vec<ResizeFilter>), // todo
	RandResizeFilterWithout(Vec<ResizeFilter>), // todo

	RandImgFilter, // todo
	RandImgFilterFrom(Vec<String>), // todo
	RandImgFilterWithout(Vec<String>), // todo

	RandCompression, // todo
	RandCompressionFrom(Vec<String>), // todo
	RandCompressionWithout(Vec<String>), // todo

	RandFormat, // todo
	RandFormatFrom(Vec<SaveFormat>), // todo
	RandFormatWithout(Vec<SaveFormat>), // todo
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum InputType {
	Path(String),
	PathArr(Vec<String>),
	PathFile // todo
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
