use crate::processor::image::Image;
use crate::config::*;

use self::image::ImageModifier;

use anyhow::Result;
use anyhow::anyhow;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use rayon::prelude::*;
use num_traits::AsPrimitive;

use indicatif::ProgressStyle;
use indicatif::ProgressBar;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub mod image;
mod parameter;
mod input;

pub enum IdType {
	Modifier,
	Sequence,
}

impl Config {
	pub fn open<P>(path: P) -> Result<Config> where P: AsRef<Path> {
		let mut file = File::open(path)?;
		let mut string = String::new();
		file.read_to_string(&mut string)?;

		let config: Config = serde_yaml::from_str(&string)?;
		Ok(config)
	}
	pub fn get_modifier<S>(&self, id: S) -> Option<Box<dyn ImageModifier>> where S: AsRef<str> {
		let image_section = self.image_section();
		let id = id.as_ref();

		let compression = image_section.compression();
		let filter = image_section.filter();
		let adjustment = image_section.adjustment();

		if let Some(compression_vector) = compression {
			for image_compression in compression_vector {
				let compression = image_compression.get();

				if compression.id() == id {
					return Some(compression);
				}
			}
		}

		if let Some(filter_vector) = filter {
			for image_filter in filter_vector {
				let filter = image_filter.get();

				if filter.id() == id {
					return Some(filter);
				}
			}
		}

		if let Some(adjustment_vector) = adjustment {
			for image_adjustment in adjustment_vector {
				let adjustment = image_adjustment.get();

				if adjustment.id() == id {
					return Some(adjustment);
				}
			}
		}

		None
	}
	fn get_sequence<S>(&self, id: S) -> Option<&Sequence> where S: AsRef<str> {
		let sequence_section = self.sequence();

		if let Some(sequence_vec) = sequence_section {
			for sequence in sequence_vec {
				if sequence.id() == id.as_ref() {
					return Some(sequence);
				}
			}
		}

		None
	}
	fn get_type<S>(&self, id: S) -> Result<IdType> where S: AsRef<str> {
		let id = id.as_ref();

		let modifier = self.get_modifier(id);
		let sequence = self.get_sequence(id);

		if let Some(_) = modifier {
			return Ok(IdType::Modifier);
		}

		if let Some(_) = sequence {
			return Ok(IdType::Sequence);
		}

		return Err(anyhow!("Unknown ID or IdType"));
	}
	fn unwrap_id<S>(&self, id: S) -> Result<Vec<String>> where S: AsRef<str> {
		let id = id.as_ref();

		let mut unwrapped = Vec::<String>::new();
		let id_type = self.get_type(id)?;

		match id_type {
			IdType::Modifier => unwrapped.push(id.to_owned()),
			IdType::Sequence => {
				let sequence = self.get_sequence(id).unwrap();

				for elem in sequence.elements() {
					for id in elem.id_seq()? {
						unwrapped.extend(self.unwrap_id(id)?);
					}
				}
			}
		}

		Ok(unwrapped)
	}
	fn init_progress_bar(&self, len: u64) -> Result<ProgressBar> {
		let progress_bar = self.progress_bar();

		let style = match progress_bar {
			Some(progress_bar) => {
				let template = progress_bar.template();
				let chars = progress_bar.chars();

				ProgressStyle::with_template(template)?.progress_chars(chars)
			}

			None => {
				let template = " Elapsed: {elapsed} {wide_bar} {percent}% | Files count: {pos}/{len} ";
				let chars = "->·";

				ProgressStyle::with_template(template)?.progress_chars(chars)
			}
		};

		let progress = ProgressBar::new(len);
		progress.set_style(style);
		progress.abandon();

		Ok(progress)
	}
	fn process_image<P>(&self, path: P) -> Result<()> where P: AsRef<Path> {
		let output = self.output().path();
		let mut image = Image::new(path)?;
		let mut id_seq = Vec::new();

		for id in self.execute()? {
			id_seq.extend(self.unwrap_id(id)?);
		}

		for id in id_seq.iter() {
			let modifier = self.get_modifier(id);

			if let Some(modifier) = modifier {
				modifier.apply(&mut image)?;
				continue;
			}

			return Err(anyhow!("Unknown ID"));
		}

		image.save(output)?;

		Ok(())
	}
	pub fn start_parallel_processing(&self) -> Result<()> {
		let path_vector = self.input().receive().files()?;
		let in_error = AtomicBool::new(false);
		let progress = self.init_progress_bar(path_vector.len().as_())?;

		let results: Vec<Result<(), anyhow::Error>> = path_vector
			.par_iter()
			.map(|path| {
				if !in_error.load(Ordering::Relaxed) {
					let result = self.process_image(path);

					if let Err(err) = result {
						in_error.store(true, Ordering::Relaxed);
						return Err(err);
					}

					progress.inc(1);
				}

				Ok(())
			})
			.collect();

		for result in results {
			result?;
		}

		Ok(())
	}
}
