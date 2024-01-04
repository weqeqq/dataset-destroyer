use crate::config::SaveFormat;

use image::io::Reader as ImageReader;
use image::GenericImageView;
use image::GrayImage;
use image::ColorType;
use image::RgbImage;
use anyhow::Result;

use std::io::Read;
use std::io::Cursor;
use std::fs::File;
use std::path::Path;

pub struct Image {
	luma: GrayImage,
	rgb: RgbImage,
	width: u32,
	height: u32,
	filename: String,
	color: ColorType,
}

impl Image {
	pub fn new<P>(path: P) -> Result<Image> where P: AsRef<Path> {
		let path = path.as_ref();

		let mut file = File::open(path)?;
		let mut buffer = Vec::new();

		file.read_to_end(&mut buffer)?;
		let image = ImageReader::new(Cursor::new(buffer)).with_guessed_format()?.decode()?;

		let color = image.color();
		let (width, height) = image.dimensions();

		let luma = image.to_luma8();
		let rgb = image.into_rgb8();

		let filename = path
			.file_name()
			.and_then(|filename| filename.to_str())
			.and_then(|filename| Some(filename.to_string()))
			.unwrap();

		Ok(Image { luma, rgb, width, height, filename, color })
	}
	pub fn luma(&self) -> &GrayImage {
		&self.luma
	}
	pub fn luma_mut(&mut self) -> &mut GrayImage {
		&mut self.luma
	}
	pub fn rgb(&self) -> &RgbImage {
		&self.rgb
	}
	pub fn rgb_mut(&mut self) -> &mut RgbImage {
		&mut self.rgb
	}
	pub fn separate_rgb_channels() {
		todo!()
	}
	pub fn width(&self) -> u32 {
		self.width
	}
	pub fn height(&self) -> u32 {
		self.height
	}
	pub fn color(&self) -> ColorType {
		self.color
	}
	pub fn save<P>(&self, path: P, save_format: SaveFormat) -> Result<()> where P: AsRef<Path> {
		let output_directory = path.as_ref().to_path_buf();
		let filename: &str = self.filename.as_ref();

		let mut output_path = output_directory.join(filename);
		match save_format {
			SaveFormat::Jpeg => {
				output_path.set_extension("jpeg");
			}
			SaveFormat::Png => {
				output_path.set_extension("png");
			}
			SaveFormat::Webp => {
				output_path.set_extension("webp");
			}
			SaveFormat::Original => (),
		}

		match self.color {
			ColorType::L8 => self.luma.save(output_path)?,
			ColorType::Rgb8 => self.rgb.save(output_path)?,
			ColorType::Rgba8 => self.rgb.save(output_path)?,
			_ => todo!(),
		}

		Ok(())
	}
}
