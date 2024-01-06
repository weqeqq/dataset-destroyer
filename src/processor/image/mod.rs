use image::RgbImage;
use image::GrayImage;
use image::RgbaImage;
use image::GrayAlphaImage;
use image::DynamicImage;

use image::ColorType;
use image::ColorType::*;
use image::ImageFormat;
use image::io::Reader;

use image::Pixel;
use image::Luma;
use image::LumaA;
use image::Rgb;
use image::Rgba;

use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use std::path::Path;

use std::ffi::OsStr;
use std::ffi::OsString;

use anyhow::Result;
use anyhow::anyhow;

pub mod filter;
pub mod compression;
pub mod adjustment;
pub mod operation;

pub trait Modifier {
	fn apply(&self, image: &mut Image) -> Result<()>;
	fn id(&self) -> &str;
}

pub struct Image {
	source: DynamicImage,
	color: ColorType,
	filename: OsString,
	format: ImageFormat,
	width: u32,
	height: u32,
}

impl Image {
	pub fn new<P>(path: P) -> Result<Image> where P: AsRef<Path> {
		let path = path.as_ref();
		let filename = path.file_name().ok_or(anyhow!("failed to get file name"))?.to_owned();

		let mut file = File::open(path)?;
		let mut buffer = Vec::new();
		file.read_to_end(&mut buffer)?;

		let reader = Reader::new(Cursor::new(buffer)).with_guessed_format()?;
		let format = reader.format().ok_or(anyhow!("failed to get image format"))?;
		let image = reader.decode()?;

		let color = image.color();
		let width = image.width();
		let height = image.height();

		let image = Image { source: image, color, filename, format, width, height };

		Ok(image)
	}
	fn separate_luma(&self) -> Result<Vec<GrayImage>> {
		let luma = self.source.as_luma8();

		if let Some(luma) = luma {
			Ok(vec![luma.clone()])
		} else {
			Err(anyhow!("image is not luma"))
		}
	}
	fn separate_luma_alpha(&self) -> Result<Vec<GrayImage>> {
		let luma_alpha = self.source.as_luma_alpha8();

		if let Some(luma_alpha) = luma_alpha {
			let width = self.width;
			let height = self.height;

			let mut luma = GrayImage::new(width, height);
			let mut alpha = GrayImage::new(width, height);

			for (x, y, pixel) in luma_alpha.enumerate_pixels() {
				let channels = pixel.channels();

				let luma_pix = Luma([channels[0]]);
				let alpha_pix = Luma([channels[1]]);

				luma.put_pixel(x, y, luma_pix);
				alpha.put_pixel(x, y, alpha_pix);
			}

			Ok(vec![luma, alpha])
		} else {
			Err(anyhow!("image is not luma alpha"))
		}
	}
	fn separate_rgb(&self) -> Result<Vec<GrayImage>> {
		let rgb = self.source.as_rgb8();

		if let Some(rgb) = rgb {
			let width = self.width;
			let height = self.height;

			let mut red = GrayImage::new(width, height);
			let mut green = GrayImage::new(width, height);
			let mut blue = GrayImage::new(width, height);

			for (x, y, pixel) in rgb.enumerate_pixels() {
				let channels = pixel.channels();

				let red_pix = Luma([channels[0]]);
				let green_pix = Luma([channels[1]]);
				let blue_pix = Luma([channels[2]]);

				red.put_pixel(x, y, red_pix);
				green.put_pixel(x, y, green_pix);
				blue.put_pixel(x, y, blue_pix);
			}

			Ok(vec![red, green, blue])
		} else {
			Err(anyhow!("image is not rgb8"))
		}
	}
	fn separate_rgba(&self) -> Result<Vec<GrayImage>> {
		let rgba = self.source.as_rgba8();

		if let Some(rgba) = rgba {
			let width = self.width;
			let height = self.height;

			let mut red = GrayImage::new(width, height);
			let mut green = GrayImage::new(width, height);
			let mut blue = GrayImage::new(width, height);
			let mut alpha = GrayImage::new(width, height);

			for (x, y, pixel) in rgba.enumerate_pixels() {
				let channels = pixel.channels();

				let red_pix = Luma([channels[0]]);
				let green_pix = Luma([channels[1]]);
				let blue_pix = Luma([channels[2]]);
				let alpha_pix = Luma([channels[3]]);

				red.put_pixel(x, y, red_pix);
				green.put_pixel(x, y, green_pix);
				blue.put_pixel(x, y, blue_pix);
				alpha.put_pixel(x, y, alpha_pix);
			}

			Ok(vec![red, green, blue, alpha])
		} else {
			Err(anyhow!("image is not rgba8"))
		}
	}
	pub fn separate_channels(&self) -> Result<Vec<GrayImage>> {
		match self.color {
			L8 => self.separate_luma(),
			La8 => self.separate_luma_alpha(),
			Rgb8 => self.separate_rgb(),
			Rgba8 => self.separate_rgba(),
			_ => unimplemented!(),
		}
	}
	fn combine_luma(&self, channels: Vec<GrayImage>) -> DynamicImage {
		DynamicImage::ImageLuma8(channels[0].clone())
	}
	fn combine_luma_alpha(&self, channels: Vec<GrayImage>) -> DynamicImage {
		let width = self.width;
		let height = self.height;

		let mut luma_alpha = GrayAlphaImage::new(width, height);

		let luma = &channels[0];
		let alpha = &channels[1];

		for (x, y, pixel) in luma_alpha.enumerate_pixels_mut() {
			let luma_pix = luma.get_pixel(x, y).channels();
			let alpha_pix = alpha.get_pixel(x, y).channels();

			let luma_alpha_pix = LumaA([luma_pix[0], alpha_pix[0]]);
			*pixel = luma_alpha_pix;
		}

		DynamicImage::ImageLumaA8(luma_alpha)
	}
	fn combine_rgb(&self, channels: Vec<GrayImage>) -> DynamicImage {
		let width = self.width;
		let height = self.height;

		let mut rgb = RgbImage::new(width, height);

		let red = &channels[0];
		let green = &channels[1];
		let blue = &channels[2];

		for (x, y, pixel) in rgb.enumerate_pixels_mut() {
			let red_pix = red.get_pixel(x, y).channels();
			let green_pix = green.get_pixel(x, y).channels();
			let blue_pix = blue.get_pixel(x, y).channels();

			let rgb_pix = Rgb([red_pix[0], green_pix[0], blue_pix[0]]);
			*pixel = rgb_pix;
		}

		DynamicImage::ImageRgb8(rgb)
	}
	fn combine_rgba(&self, channels: Vec<GrayImage>) -> DynamicImage {
		let width = self.width;
		let height = self.height;

		let mut rgba = RgbaImage::new(width, height);

		let red = &channels[0];
		let green = &channels[1];
		let blue = &channels[2];
		let alpha = &channels[3];

		for (x, y, pixel) in rgba.enumerate_pixels_mut() {
			let red_pix = red.get_pixel(x, y).channels();
			let green_pix = green.get_pixel(x, y).channels();
			let blue_pix = blue.get_pixel(x, y).channels();
			let alpha_pix = alpha.get_pixel(x, y).channels();

			let rgba_pix = Rgba([red_pix[0], green_pix[0], blue_pix[0], alpha_pix[0]]);
			*pixel = rgba_pix;
		}

		DynamicImage::ImageRgba8(rgba)
	}
	pub fn combine_channels(&self, channels: Vec<GrayImage>) -> DynamicImage {
		match self.color {
			L8 => self.combine_luma(channels),
			La8 => self.combine_luma_alpha(channels),
			Rgb8 => self.combine_rgb(channels),
			Rgba8 => self.combine_rgba(channels),
			_ => unimplemented!(),
		}
	}
	pub fn save<P>(&self, path: P) -> Result<()> where P: AsRef<Path> {
		let path = path.as_ref();
		let filename: &OsStr = self.filename.as_ref();

		self.source.save(path.join(filename))?;
		Ok(())
	}
	pub fn source(&self) -> &DynamicImage {
		&self.source
	}
	pub fn source_mut(&mut self) -> &mut DynamicImage {
		&mut self.source
	}
	pub fn filename(&self) -> &str {
		self.filename.to_str().unwrap()
	}
	pub fn width(&self) -> u32 {
		self.width
	}
	pub fn height(&self) -> u32 {
		self.height
	}
	pub fn format(&self) -> ImageFormat {
		self.format
	}
	pub fn color(&self) -> ColorType {
		self.color
	}
}
