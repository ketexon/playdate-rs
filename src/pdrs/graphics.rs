pub mod font;

use std::{ffi::{CString, NulError}, mem::MaybeUninit};

use crate::{
	api::{
		graphics::{PlaydateGraphics, LCDPattern, LCDColor, PDStringEncoding, make_opaque_pattern, LCDBitmap}
	}
};

pub use crate::{
	api::{
		graphics::{
			LCDSolidColor as SolidColor,
			LCD_ROWS,
			LCD_COLUMNS,
		}
	}
};

use self::font::Font;

pub const SCREEN_SIZE: (i32, i32) = (LCD_COLUMNS, LCD_ROWS);

pub enum FontVariant {
	Normal,
	Bold,
	Italic
}

type OpaquePattern = [u8;8];

#[derive(Clone, Copy)]
pub struct Pattern(LCDPattern);

#[derive(Clone, Copy)]
pub enum Color {
	Solid(SolidColor),
	Pattern(Pattern),
}

impl Pattern {
	pub fn new(pattern: [u8;16]) -> Self {
		Self(pattern)
	}

	pub fn opaque(pattern: OpaquePattern) -> Self {
		Self(make_opaque_pattern(pattern))
	}

	pub fn solid(color: SolidColor) -> Self {
		match color {
			SolidColor::White => Self::opaque([0xff;8]),
			SolidColor::Black => Self::opaque([0x00;8]),
			SolidColor::Clear => Self([0;16]),
			SolidColor::XOR => panic!("Pattern::solid does not support SolidColor::XOR")
		}
	}

	pub fn invert(&mut self) {
		for i in 0..8 {
			self.0[i as usize] = !self.0[i as usize];
		}
	}

	pub fn checkerboard(w: u8) -> Self {
		let mut pattern = [0u8;8];
		if w != 0 {
			for i in 0..8 {
				for j in 0..8 {
					if i/w % 2 == j/w % 2 {
						pattern[i as usize] |= 1 << j;
					}
				}
			}
		}

		Self::opaque(pattern)
	}

	pub fn horizontal_stripes(h: u8) -> Self {
		let mut pattern = [0;8];
		if h != 0 {
			for i in 0..8 {
				if i/h % 2 == 0 {
					pattern[i as usize] = 0xffu8;
				}
			}
		}

		Self::opaque(pattern)
	}


	pub fn vertial_stripes(w: u8) -> Self {
		let mut pattern = [0;8];
		if w != 0 {
			for i in 0..8 {
				if i/w % 2 == 0 {
					pattern[i as usize] = 0xffu8;
				}
			}
		}

		Self::opaque(pattern)
	}

	pub fn diagonal_stripes(w: u8) -> Self {
		if w == 1 {
			Self::checkerboard(1)
		}
		else if w == 2 {
			Self::opaque([
				0b10011001u8,
				0b11001100u8,
				0b01100110u8,
				0b00110011u8,
				0b10011001u8,
				0b11001100u8,
				0b01100110u8,
				0b00110011u8,
			])
		}
		else if w == 4 {
			Self::opaque([
				0b10000111u8,
				0b11000011u8,
				0b11100001u8,
				0b11110000u8,
				0b01111000u8,
				0b00111100u8,
				0b00011110u8,
				0b00001111u8,
			])
		}
		else if w == 0 {
			Self::solid(SolidColor::Black)
		}
		else if w == 8 {
			Self::solid(SolidColor::White)
		}
		else {
			panic!("w must be a multiple of 2");
		}
	}


	pub fn flip_horizontal(&mut self){
		for b in &mut self.0 {
			*b = (*b & 0xF0u8) >> 4 | (*b & 0x0Fu8) << 4;
			*b = (*b & 0xCCu8) >> 2 | (*b & 0x33u8) << 2;
			*b = (*b & 0xAAu8) >> 1 | (*b & 0x55u8) << 1;
		}
	}

	pub fn flip_vertical(&mut self){
		for i in 0..4 {
			self.0.swap(i, 7-i);
			self.0.swap(7+i, 15-i);
		}
	}

	pub fn rotate_90(&mut self){
		let mut new_pattern = [0u8;16];
		for c in 0..8u8 {
			new_pattern[c as usize] = self.get_column(c, false);
			new_pattern[c as usize + 8] = self.get_column(c, true);
		}
		self.0 = new_pattern;
	}

	pub fn rotate_180(&mut self) {
		self.flip_horizontal();
		self.flip_vertical();
	}

	pub fn get_column(&self, c: u8, mask: bool) -> u8 {
		let mut out = 0u8;
		for r in 0..8 {
			out |= if self.0[r as usize + if mask {7} else {0}] & (1 << (7-c)) > 0 {
				1 << r
			}
			else {
				0
			};
		}
		out
	}
}

// impl From<

impl Color {
	pub fn invert(&mut self) {
		match self {
			Self::Solid(ref mut color) => {
				*color = match color.clone() {
					SolidColor::Black => SolidColor::White,
					SolidColor::White => SolidColor::Black,
					other => other
				};
			},
			Self::Pattern(ref mut pattern) => {
				pattern.invert()
			}
		};
	}
}

impl From<SolidColor> for Color {
	fn from(value: SolidColor) -> Self {
		Self::Solid(value)
	}
}

impl From<LCDPattern> for Color {
	fn from(value: LCDPattern) -> Self {
		Self::Pattern(Pattern::new(value))
	}
}

impl From<OpaquePattern> for Color {
	fn from(value: OpaquePattern) -> Self {
		Self::Pattern(Pattern::opaque(value))
	}
}

impl From<Pattern> for Color {
	fn from(value: Pattern) -> Self {
		Self::Pattern(value)
	}
}

impl From<&Color> for LCDColor {
	fn from(color: &Color) -> Self {
		match color {
			Color::Solid(solid_color) => Self { solid_color: solid_color.clone() },
			Color::Pattern(pattern) => Self { pattern: &pattern.0 }
		}
	}
}

#[derive(Debug)]
pub enum LoadFontError {
	PathNulError(NulError),
	LoadError(String),
}

pub struct Graphics(PlaydateGraphics);

impl Graphics {
	pub const SYSTEM_FONT_PATH: &'static str = "/System/Fonts/";
	pub const DEFAULT_SYSTEM_FONT_NORMAL: &'static str = "Asheville-Sans-14-Light.pft";
	pub const DEFAULT_SYSTEM_FONT_BOLD: &'static str = "Asheville-Sans-14-Bold.pft";
	pub const DEFAULT_SYSTEM_FONT_ITALICS: &'static str = "Asheville-Sans-14-Light-Oblique.pft";
	pub const DEFAULT_SYSTEM_FONT_LARGE: &'static str = "Asheville-Sans-24-Light.pft";

	pub fn api(&self) -> &PlaydateGraphics {
		&self.0
	}

	/*--------------- DRAWING FUNCTIONS ------------------*/
	pub fn clear(&self, color: Color) {
		(self.0.clear)((&color).into());
	}

	pub fn set_background_color(&self, color: SolidColor){
		(self.0.set_background_color)(color);
	}

	pub fn draw_text<T: Into<Vec<u8>>>(&self, s: T, pos: na::Point2<i32>) {
		self.draw_text_checked(s, pos).unwrap();
	}

	pub fn draw_text_checked<T: Into<Vec<u8>>>(&self, s: T, pos: na::Point2<i32>) -> Result<(), NulError> {
		let s_vec: Vec<u8> = s.into();
		let len = s_vec.len();
		CString::new(s_vec)
            .map(|cstring|{
                unsafe {
					(self.0.draw_text)(
						std::mem::transmute(cstring.as_ptr()),
						len,
						PDStringEncoding::ASCII,
						pos.x, pos.y
					);
				};
            })
	}

	pub fn load_font<T: Into<Vec<u8>>>(&self, path: T) -> Result<Font, LoadFontError> {
		CString::new(path)
			.map_err(|nul_error| {LoadFontError::PathNulError(nul_error)})
            .and_then(|cstring|{
                unsafe {
					let mut error: MaybeUninit<*const char> = MaybeUninit::uninit();
					let font = (self.0.load_font)(
						std::mem::transmute(cstring.as_ptr()),
						error.as_mut_ptr()
					);
					if font.is_null() {
						Err(
							LoadFontError::LoadError(
								CString::from_raw(std::mem::transmute(error))
									.into_string()
									.expect("Invalid error string returned from load_font")
							)
						)
					}
					else {
						Ok(Font::new(font, &self))
					}
				}
            })
	}

	pub fn get_system_font_checked(&self, variant: FontVariant) -> Result<Font, LoadFontError> {
		self.load_font(
			String::from(Self::SYSTEM_FONT_PATH) + match variant {
				FontVariant::Normal => Self::DEFAULT_SYSTEM_FONT_NORMAL,
				FontVariant::Bold => Self::DEFAULT_SYSTEM_FONT_BOLD,
				FontVariant::Italic => Self::DEFAULT_SYSTEM_FONT_ITALICS,
			}
		)
	}

	pub fn get_system_font(&self, variant: FontVariant) -> Font {
		self.get_system_font_checked(variant).unwrap()
	}

	pub fn get_large_system_font_checked(&self) -> Result<Font, LoadFontError> {
		self.load_font(
			String::from(Self::SYSTEM_FONT_PATH) + Self::DEFAULT_SYSTEM_FONT_LARGE
		)
	}

	pub fn get_large_system_font(&self) -> Font {
		self.get_large_system_font_checked().unwrap()
	}
}