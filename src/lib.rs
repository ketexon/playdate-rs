pub extern crate nalgebra as na;

use na::{Vector2, Point2};
use pdrs::{graphics::{SolidColor, font::Font, Color, Pattern}};

pub(crate) mod api;
pub(crate) mod pdrs;
use api::{
    PlaydateAPI,
    event::{SystemEvent}, graphics::{LCDColor},
};

use pdrs::playdate::Playdate;

use pdrs::graphics::{FontVariant, SCREEN_SIZE};

#[repr(C)]
pub enum SceneData {
    Main{
        start_time: u32,
        text: &'static str,
        text_pos: Point2<f32>,
    },
}

#[repr(C)]
pub struct UserData {
    pub pd: &'static Playdate,
    pub font: Font<'static>,
    pub scene: SceneData
}

#[no_mangle]
pub unsafe extern "C" fn eventHandler(
    pd: &'static Playdate,
    ev: SystemEvent,
    _arg: u32
) -> i32 {
    static mut USERDATA: Option<Box<UserData>> = None;
    match ev {
        SystemEvent::Init => {
            pd.init_allocator();

            let text = "Hello World!\nHello there!";

            let mut font = pd.graphics.get_system_font(FontVariant::Normal);
            font.height_offset = -4;

            // let font_height = (pd.graphics.api().get_font_height)(font);
            let (text_w, text_h) = font.get_text_size(text).unwrap();
            pd.print(format!("{text_w}x{text_h}"));

            USERDATA = Some(Box::new(UserData {
                pd,
                font,
                scene: SceneData::Main {
                    start_time: pd.system.get_unix_time_seconds(),
                    text,
                    text_pos: Point2::new(
                        ((SCREEN_SIZE.0 - text_w)/2) as f32,
                        ((SCREEN_SIZE.1 - text_h as i32)/2) as f32
                    ),
                }
            }));
            pd.system.set_update_callback(update, USERDATA.as_mut().unwrap());
        }
        _ => {
        }
    };
    0
}

#[allow(unused_variables)]
pub extern "C" fn update(mut userdata: &mut UserData) -> i32 {
    let UserData {pd, font, scene} = &mut userdata;

    match scene {
        SceneData::Main {
            start_time,
            text,
            ref mut text_pos,
            ..
        } => {
            let (s, ms) = pd.system.get_unix_time_seconds_milliseconds();

            (*font).leading_adjustment = (s - *start_time) as i32;
            font.activate();
            let (text_size_x, text_size_y) = font.get_text_size(*text).unwrap();

            let mut color = Color::from(Pattern::opaque([
                0b10000000,
                0b11000000,
                0b11100000,
                0b11110000,
                0b11111000,
                0b11111100,
                0b11111110,
                0b11111111,
            ]));
            if s % 2 == 0  {
                if let Color::Pattern(ref mut pattern) = color {
                    pattern.rotate_90();
                }
            }
            // pd.graphics.clear(SolidColor::White.into());
            pd.graphics.clear(color);


            *text_pos += na::Scale2::new(1f32, -1f32) * pd.system.get_dpad_axes_normalized();

            pd.graphics.draw_text(text.as_bytes(), Point2::from(text_pos.coords.try_cast().unwrap()));
            (pd.graphics.api().draw_rect)(
                text_pos.x as i32,
                text_pos.y as i32,
                text_size_x,
                text_size_y,
                LCDColor{solid_color: SolidColor::Black}
            );
            pd.system.draw_fps(Vector2::new(0,0));
        }
    }

    // pd.sys().api().get_button_state
    1
}