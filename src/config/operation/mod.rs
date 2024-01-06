use serde::Serialize;
use serde::Deserialize;
use super::enumerations::*;

mod interface;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Resize {
	pub id: String,
	width: Option<Parameter>,
	height: Option<Parameter>,
	filter: ResizeFilter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ToLumaAlpha {
	pub id: String 
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ToLuma {
	pub id: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ToRgba {
	pub id: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ToRgb {
	pub id: String,
}
