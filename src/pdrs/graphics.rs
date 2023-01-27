use std::ffi::{CString, NulError};

use crate::{
	api::{
		PlaydateAPI,
		graphics::{PlaydateGraphics, LCDSolidColor, LCDPattern, LCDColor, PDStringEncoding}
	}
};

pub use crate::{
	api::{
		graphics::{
			LCDSolidColor as SolidColor,
		}
	}
};

pub enum Color {
	Solid(SolidColor),
	Pattern(LCDPattern),
}

impl From<&Color> for LCDColor {
	fn from(color: &Color) -> Self {
		match color {
			Color::Solid(solid_color) => Self { solid_color: solid_color.clone() },
			Color::Pattern(pattern) => Self { pattern: pattern }
		}
	}
}

pub struct Graphics(PlaydateGraphics);

impl Graphics {
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

	pub fn draw_text<T: Into<Vec<u8>>>(&self, s: T, pos: na::Vector2<i32>) {
		self.draw_text_checked(s, pos).unwrap();
	}

	pub fn draw_text_checked<T: Into<Vec<u8>>>(&self, s: T, pos: na::Vector2<i32>) -> Result<(), NulError> {
		let s_vec: Vec<u8> = s.into();
		let len = s_vec.len();
		CString::new(s_vec)
            .map(|cstring|{
                unsafe {
					(self.0.draw_text)(
						std::mem::transmute(cstring.into_raw()),
						len,
						PDStringEncoding::ASCII,
						pos.x, pos.y
					)
				};
            })
	}
}