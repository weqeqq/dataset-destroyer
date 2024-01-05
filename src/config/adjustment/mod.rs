use serde::Serialize;
use serde::Deserialize;
use super::enumerations::*;

mod interface;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ImageAdjustment {
	Brighten(Box<Brighten>),
	Contrast(Box<Contrast>),
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Brighten {
	pub id: String,
	value: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Contrast {
	pub id: String,
	contrast: Parameter,
}
