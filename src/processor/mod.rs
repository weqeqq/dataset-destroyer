use crate::config::*;

use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::processor::image::Image;
use self::image::filter::Filter;
use self::image::compression::Compression;
use self::image::adjustment::Adjustment;

use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressStyle;
use indicatif::ProgressFinish;

pub mod image;
mod parameter;
mod input;

impl Config {
	pub fn open<P>(path: P) -> Result<Config> where P: AsRef<Path> {
		let mut file = File::open(path)?;
		let mut string = String::new();
		file.read_to_string(&mut string)?;

		let config: Config = serde_yaml::from_str(&string)?;
		Ok(config)
	}
	fn compression_by_id<S>(&self, id: S) -> Option<Box<dyn Compression>> where S: AsRef<str> {
		let compression_section = self.image_section().compression();

		if let Some(compression_vec) = compression_section {
			for image_compression in compression_vec {
				let compression = image_compression.get();

				if compression.id() == id.as_ref() {
					return Some(compression);
				}
			}
		}

		None
	}
	fn filter_by_id<S>(&self, id: S) -> Option<Box<dyn Filter>> where S: AsRef<str> {
		let filter_section = self.image_section().filter();

		if let Some(filter_vec) = filter_section {
			for image_filter in filter_vec {
				let filter = image_filter.get();

				if filter.id() == id.as_ref() {
					return Some(filter);
				}
			}
		}

		None
	}
	fn adjustment_by_id<S>(&self, id: S) -> Option<Box<dyn Adjustment>> where S: AsRef<str> {
		let adjustment_section = self.image_section().adjustment();

		if let Some(adjustment_vec) = adjustment_section {
			for image_adjustment in adjustment_vec {
				let adjustment = image_adjustment.get();

				if adjustment.id() == id.as_ref() {
					return Some(adjustment);
				}
			}
		}

		None
	}
	fn sequence_by_id<S>(&self, id: S) where S: AsRef<str> {
		todo!()
	}
	pub fn start_processing(&self) -> Result<()> {
		let path_vec = self.input().receive().files()?;

		let progress_bar = self.progress_bar();
		let style = match progress_bar {
			Some(progress_bar) => {
				let template = progress_bar.template();
				let chars = progress_bar.chars();

				ProgressStyle::with_template(template)?.progress_chars(chars)
			}
			None => {
				let template = " Eta: {eta} {wide_bar} {percent}% | Files count: {len} ";
				let chars = "->·";

				ProgressStyle::with_template(template)?.progress_chars(chars)
			}
		};

		let results: Vec<Result<(), anyhow::Error>> = path_vec
			.par_iter()

			.progress_with_style(style)
			.with_finish(ProgressFinish::Abandon)

			.map(|path| {
				let mut image = Image::new(path)?;
				let output_path = self.output().path();
				let execution_list = self.execute();

				for id in execution_list {
					let compression = self.compression_by_id(id);
					let filter = self.filter_by_id(id);
					let adjustment = self.adjustment_by_id(id);

					if let Some(compression) = compression {
						compression.apply(&mut image)?;
						break;
					}

					if let Some(filter) = filter {
						filter.apply(&mut image)?;
						break;
					}

					if let Some(adjustment) = adjustment {
						adjustment.apply(&mut image)?;
						break;
					}
				}

				image.save(output_path)?;

				Ok(())
			})
			.collect();

		for result in results {
			result?;
		}

		Ok(())
	}
}
