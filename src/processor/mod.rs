use crate::config::enumerations::IdType;
use crate::processor::image::Image;
use crate::config::*;

use self::image::filter::Filter;
use self::image::compression::Compression;
use self::image::adjustment::Adjustment;

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

impl Config {
	pub fn open<P>(path: P) -> Result<Config> where P: AsRef<Path> {
		let mut file = File::open(path)?;
		let mut string = String::new();
		file.read_to_string(&mut string)?;

		let config: Config = serde_yaml::from_str(&string)?;
		Ok(config)
	}
	fn get_compr<S>(&self, id: S) -> Option<Box<dyn Compression>> where S: AsRef<str> {
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
	fn get_filter<S>(&self, id: S) -> Option<Box<dyn Filter>> where S: AsRef<str> {
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
	fn get_adjust<S>(&self, id: S) -> Option<Box<dyn Adjustment>> where S: AsRef<str> {
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
	fn get_seq<S>(&self, id: S) -> Option<&Sequence> where S: AsRef<str> {
		let sequence_section = self.sequence();

		if let Some(sequence_vec) = sequence_section {
			for sequence in sequence_vec {
				if sequence.id() == id.as_ref() {
					//println!("yes");
					return Some(sequence);
				}
			}
		}

		None
	}
	fn get_type<S>(&self, id: S) -> Result<IdType> where S: AsRef<str> {
		let id = id.as_ref();

		let compression = self.get_compr(id);
		let filter = self.get_filter(id);
		let adjustment = self.get_adjust(id);
		let sequence = self.get_seq(id);

		if let Some(_) = compression {
			return Ok(IdType::Compression);
		}

		if let Some(_) = filter {
			return Ok(IdType::Filter);
		}

		if let Some(_) = adjustment {
			return Ok(IdType::Adjustment);
		}

		if let Some(_) = sequence {
			return Ok(IdType::Sequence);
		}

		return Err(anyhow!("undefined id"));
	}
	fn unwrap_id<S>(&self, id: S) -> Result<Vec<String>> where S: AsRef<str> {
		let id = id.as_ref();

		let mut unwrapped = Vec::<String>::new();
		let id_type = self.get_type(id)?;

		match id_type {
			IdType::Filter | IdType::Adjustment | IdType::Compression => unwrapped.push(id.to_owned()),
			IdType::Sequence => {
				let seq = self.get_seq(id).unwrap();

				for elem in seq.elements() {
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
				let template = " Eta: {eta} {wide_bar} {percent}% | Files count: {len} {msg}";
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
			let compr = self.get_compr(id);
			let filter = self.get_filter(id);
			let adjust = self.get_adjust(id);

			if let Some(compr) = compr {
				compr.apply(&mut image)?;
				continue;
			}

			if let Some(filter) = filter {
				filter.apply(&mut image)?;
				continue;
			}

			if let Some(adjust) = adjust {
				adjust.apply(&mut image)?;
				continue;
			}

			return Err(anyhow!("proc error"));
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
