use crate::config::enumerations::InputType;
use anyhow::Result;
use std::path::PathBuf;
use glob::glob;

impl InputType {
	pub fn files(&self) -> Result<Vec<PathBuf>> {
		let string_vec = match self {
			Self::Path(s) => vec![s.clone()],
			Self::PathArr(s) => s.clone(),
			Self::PathFile => todo!(),
		};

		let mut path_vec = Vec::new();

		for string in string_vec {
			let glob = glob(&string)?;

			for path in glob {
				let path = path?;

				path_vec.push(path);
			}
		}

		Ok(path_vec)
	}
}
