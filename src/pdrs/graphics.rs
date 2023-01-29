pub mod font;

use std::{ffi::{CString, NulError}, mem::MaybeUninit};

use crate::{
	api::{
		graphics::{PlaydateGraphics, LCDPattern, LCDColor, PDStringEncoding}
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

pub enum Color {
	Solid(SolidColor),
	Pattern(LCDPattern),
}

impl From<SolidColor> for Color {
	fn from(value: SolidColor) -> Self {
		Self::Solid(value)
	}
}

impl From<&Color> for LCDColor {
	fn from(color: &Color) -> Self {
		match color {
			Color::Solid(solid_color) => Self { solid_color: solid_color.clone() },
			Color::Pattern(pattern) => Self { pattern: pattern }
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