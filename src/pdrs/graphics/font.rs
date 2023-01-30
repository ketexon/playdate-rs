use std::ffi::{CString, NulError};

use crate::{
    api::{
        graphics::LCDFont
    },
    pdrs::{
        graphics::Graphics,
    }
};

pub struct Font<'a> {
    lcd_font: *const LCDFont,
    graphics: &'a Graphics,
    pub tracking: i32,
    pub height_offset: i32,
    pub leading_adjustment: i32,
}


impl<'a> Font<'a> {
    pub fn new(lcd_font: *const LCDFont, graphics: &'a Graphics) -> Self{
        Self{lcd_font, graphics, tracking: 0, height_offset: 0, leading_adjustment: 0}
    }

    pub fn get_text_size<T: Into<Vec<u8>>>(&self, s: T) -> Result<(i32, i32), NulError> {
        let s_vec: Vec<u8> = s.into();
        let len = s_vec.len();
        let n_lines = s_vec.iter().filter(|ch| {**ch == '\n' as u8}).collect::<Vec<&u8>>().len() as i32;
        let height = (
            (self.graphics.api().get_font_height)(self.lcd_font) as i32
            + self.height_offset
        ) * (n_lines + 1) + self.leading_adjustment * n_lines;
        CString::new(s_vec)
            .map(|cstring: CString|{
                unsafe {
                    (self.graphics.api().get_text_width)(
                        self.lcd_font,
                        std::mem::transmute(cstring.as_ptr()),
                        len,
                        crate::api::graphics::PDStringEncoding::ASCII,
                        self.tracking
                    )
                }
            })
            .map(|width|{
                (width, height)
            })
    }

    pub fn activate(&self){
        (self.graphics.api().set_font)(self.lcd_font);
        (self.graphics.api().set_text_tracking)(self.tracking);
        (self.graphics.api().set_text_leading)(self.leading_adjustment + self.height_offset);
    }

    pub fn draw_text_checked<T: Into<Vec<u8>>>(&self, s: T, pos: na::Point2<i32>) -> Result<(), NulError> {
        self.activate();
        self.graphics.draw_text_checked(s, pos)
    }

    pub fn draw_text<T: Into<Vec<u8>>>(&self, s: T, pos: na::Point2<i32>) {
        self.draw_text_checked(s, pos).unwrap()
    }
}


impl<'a> From<&Font<'a>> for *const LCDFont {
    fn from(font: &Font) -> Self {
        font.lcd_font
    }
}