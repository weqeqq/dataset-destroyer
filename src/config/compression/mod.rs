use serde::Serialize;
use serde::Deserialize;
use super::enumerations::*;

mod interface;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ImageCompression {
	Jpeg(Box<Jpeg>),
	WebP(Box<WebP>),
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Jpeg {
	pub id: String,
	quality: Parameter,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct WebP {
	pub id: String,
	quality: Parameter,
}
