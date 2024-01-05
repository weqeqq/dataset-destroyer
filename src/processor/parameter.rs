use crate::config::enumerations::Parameter;
use rand::prelude::*;
use anyhow::Result;
use anyhow::anyhow;
use num_traits::AsPrimitive;

impl Parameter {
	pub fn int(&self) -> Result<u32> {
		let mut rng = thread_rng();

		match self {
			Self::Int(i) => Ok(*i),

			Self::RandInt(s, e) => Ok(rng.gen_range(*s..*e).as_()),

			Self::RandIntFrom(i) =>
				i
					.choose(&mut rng)
					.and_then(|i| Some(*i))
					.ok_or(anyhow!("int")),

			_ => Err(anyhow!("int")),
		}
	}
	pub fn float(&self) -> Result<f32> {
		let mut rng = thread_rng();

		match self {
			Self::Float(f) => Ok(*f),

			Self::RandFloat(s, e) => Ok(rng.gen_range(*s..*e)),

			Self::RandFloatFrom(f) =>
				f
					.choose(&mut rng)
					.and_then(|f| Some(*f))
					.ok_or(anyhow!("float")),

			_ => Err(anyhow!("float")),
		}
	}
	pub fn id_seq(&self) -> Result<Vec<&str>> {
		let mut rng = thread_rng();

		match self {
			Self::Id(i) => Ok(vec![i]),

			Self::RandId(i) =>
				i
					.choose(&mut rng)
					.and_then(|s| Some(vec![s.as_ref()]))
					.ok_or(anyhow!("not id")),

			Self::RandMulId(i, amount) =>
				i
					.choose_multiple(&mut rng, *amount)
					.map(|s| Ok(s.as_str()))
					.collect(),

			_ => Err(anyhow!("not id")),
		}
	}
}
