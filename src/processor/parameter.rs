use crate::config::enumerations::Parameter;
use anyhow::Result;
use anyhow::anyhow;

impl Parameter {
	pub fn int(&self) -> Result<u32> {
		match self {
			Self::Int(i) => Ok(*i),
			_ => Err(anyhow!("expected integer value")),
		}
	}
	pub fn float(&self) -> Result<f32> {
		match self {
			Self::Float(f) => Ok(*f),
			_ => Err(anyhow!("expected float value")),
		}
	}
}
