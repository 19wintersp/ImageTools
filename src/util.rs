#![allow(dead_code)]

use std::str::FromStr;
use color_processing::Color;
use image::{ DynamicImage, ImageFormat, ImageOutputFormat, Pixel, Rgb, Rgba };
use image::imageops::FilterType;
use image::codecs::pnm::{ PNMSubtype, SampleEncoding };

#[derive(Clone, Copy, Debug)]
pub enum ColorFormat {
	Hex,
	Cmyk,
	Hsl,
	Hsv,
	Hwb,
	Rgb,
}

impl FromStr for ColorFormat {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use ColorFormat::*;

		match s.to_lowercase().as_str() {
			"hex" => Ok(Hex),
			"cmyk" => Ok(Cmyk),
			"hsl" | "hsla" => Ok(Hsl),
			"hsv" | "hsva" => Ok(Hsv),
			"hwb" | "hwba" => Ok(Hwb),
			"rgb" | "rgba" => Ok(Rgb),
			_ => Err(()),
		}
	}
}

pub fn color_to_string(color: Color, format: ColorFormat) -> String {
	use ColorFormat::*;

	match format {
		Hex => color.to_hex_string(),
		Cmyk => color.to_cmyk_string(),
		Hsl => color.to_hsl_string(),
		Hsv => color.to_hsv_string(),
		Hwb => color.to_hwb_string(),
		Rgb => color.to_rgb_string(),
	}
}

pub fn color_to_rgb(color: Color) -> Rgb<u8> {
	Rgb([ color.red, color.green, color.blue ])
}

pub fn color_to_rgba(color: Color) -> Rgba<u8> {
	Rgba([ color.red, color.green, color.blue, color.alpha ])
}

pub fn pixel_to_color<P: Pixel<Subpixel = u8>>(pixel: P) -> Color {
	let rgba = pixel.to_rgba();
	Color::new_rgba(rgba[0], rgba[1], rgba[2], rgba[3])
}

pub fn format_from_string(string: String) -> Option<ImageFormat> {
	match string.to_ascii_lowercase().as_str() {
		"png" => Some(ImageFormat::Png),
		"gif" => Some(ImageFormat::Gif),
		"webp" => Some(ImageFormat::WebP),
		"tiff" => Some(ImageFormat::Tiff),
		"tga" => Some(ImageFormat::Tga),
		"dds" => Some(ImageFormat::Dds),
		"hdr" => Some(ImageFormat::Hdr),
		"avif" => Some(ImageFormat::Avif),
		"jpeg" | "jpg" => Some(ImageFormat::Jpeg),
		"bmp" | "bitmap" => Some(ImageFormat::Bmp),
		"ico" | "icon" => Some(ImageFormat::Ico),
		"pnm" | "pbm" | "pgm" | "pam" => Some(ImageFormat::Pnm),
		"ff" | "farbfeld" => Some(ImageFormat::Farbfeld),
		_ => None,
	}
}

pub fn oformat_from_string(string: String) -> Option<ImageOutputFormat> {
	match string.to_ascii_lowercase().as_str() {
		"png" => Some(ImageOutputFormat::Png),
		"jpeg" | "jpg" => Some(ImageOutputFormat::Jpeg(100)),
		"gif" => Some(ImageOutputFormat::Gif),
		"ico" => Some(ImageOutputFormat::Ico),
		"bmp" => Some(ImageOutputFormat::Bmp),
		"ff" | "farbfeld" => Some(ImageOutputFormat::Farbfeld),
		"tga" => Some(ImageOutputFormat::Tga),
		"pnmb" | "pbm" => Some(ImageOutputFormat::Pnm(PNMSubtype::Bitmap(SampleEncoding::Binary))),
		"pnmb-b" | "pbm-b" => Some(ImageOutputFormat::Pnm(PNMSubtype::Bitmap(SampleEncoding::Binary))),
		"pnmb-a" | "pbm-a" => Some(ImageOutputFormat::Pnm(PNMSubtype::Bitmap(SampleEncoding::Ascii))),
		"pnmg" | "pgm" => Some(ImageOutputFormat::Pnm(PNMSubtype::Graymap(SampleEncoding::Binary))),
		"pnmg-b" | "pgm-b" => Some(ImageOutputFormat::Pnm(PNMSubtype::Graymap(SampleEncoding::Binary))),
		"pnmg-a" | "pgm-a" => Some(ImageOutputFormat::Pnm(PNMSubtype::Graymap(SampleEncoding::Ascii))),
		"pnmp" | "ppm" => Some(ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Binary))),
		"pnmp-b" | "ppm-b" => Some(ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Binary))),
		"pnmp-a" | "ppm-a" => Some(ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Ascii))),
		"pnma" | "pam" => Some(ImageOutputFormat::Pnm(PNMSubtype::ArbitraryMap)),
		"pnm1" | "p1" => Some(ImageOutputFormat::Pnm(PNMSubtype::Bitmap(SampleEncoding::Binary))),
		"pnm2" | "p2" => Some(ImageOutputFormat::Pnm(PNMSubtype::Graymap(SampleEncoding::Binary))),
		"pnm3" | "p3" => Some(ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Binary))),
		"pnm4" | "p4" => Some(ImageOutputFormat::Pnm(PNMSubtype::Bitmap(SampleEncoding::Ascii))),
		"pnm5" | "p5" => Some(ImageOutputFormat::Pnm(PNMSubtype::Graymap(SampleEncoding::Ascii))),
		"pnm6" | "p6" => Some(ImageOutputFormat::Pnm(PNMSubtype::Pixmap(SampleEncoding::Ascii))),
		"pnm7" | "p7" => Some(ImageOutputFormat::Pnm(PNMSubtype::ArbitraryMap)),
		_ => None,
	}
}

pub fn filter_from_string(string: String) -> Option<FilterType> {
	match string.to_ascii_lowercase().as_str() {
		"nearest" => Some(FilterType::Nearest),
		"triangle" => Some(FilterType::Triangle),
		"gaussian" => Some(FilterType::Gaussian),
		"catmull" => Some(FilterType::CatmullRom),
		"catmullrom" => Some(FilterType::CatmullRom),
		"lanczos3" => Some(FilterType::Lanczos3),
		_ => None,
	}
}

pub fn image_type(image: DynamicImage) -> String {
	use DynamicImage::*;

	match image {
		ImageLuma8(_) => "8-bit Luma",
		ImageLumaA8(_) => "8-bit Luma with alpha",
		ImageRgb8(_) => "8-bit RGB",
		ImageRgba8(_) => "8-bit RGB with alpha",
		ImageBgr8(_) => "8-bit BGR",
		ImageBgra8(_) => "8-bit BGR with alpha",
		ImageLuma16(_) => "16-bit Luma",
		ImageLumaA16(_) => "16-bit Luma with alpha",
		ImageRgb16(_) => "16-bit RGB",
		ImageRgba16(_) => "16-bit RGB with alpha",
	}.into()
}
