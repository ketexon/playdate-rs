pub extern crate nalgebra as na;

use std::{ptr::{null_mut, null}, default};

use na::Vector2;
use pdrs::{graphics::Color};

pub(crate) mod api;
pub(crate) mod pdrs;
use api::{
    PlaydateAPI,
    event::{SystemEvent}, graphics::{LCDSolidColor}, system::PDButtons,
};

use pdrs::playdate::Playdate;

#[repr(C)]
pub enum SceneData {
    Main{text_pos: na::Vector2<f32>},
}

#[repr(C)]
pub struct UserData {
    pub pd: &'static Playdate,
    pub scene: SceneData
}

#[no_mangle]
pub unsafe extern "C" fn eventHandler(
    pd: &'static Playdate,
    // pd_raw: Playdate,
    ev: SystemEvent,
    arg: u32
) -> i32 {
    static mut USERDATA: Option<Box<UserData>> = None;
    match ev {
        SystemEvent::Init => {
            pd.init_allocator();

            USERDATA = Some(Box::new(UserData {
                pd: pd,
                scene: SceneData::Main { text_pos: Vector2::new(200f32, 150f32) }
            }));
            pd.system.set_update_callback(update0, USERDATA.as_mut().unwrap());
        }
        _ => {
        }
    };
    0
}

pub extern "C" fn update0(mut userdata: &mut UserData) -> i32 {
    let UserData {pd, scene} = &mut userdata;

    match scene {
        SceneData::Main { ref mut text_pos } => {
            let s = pd.system.get_unix_time_seconds();
            let clear_color = if s % 2 == 0 {
                Color::Solid(LCDSolidColor::Black)
            } else {
                Color::Solid(LCDSolidColor::White)
            };
            pd.graphics.clear(clear_color);

            // let (current, pushed, released) = pd.system.get_button_state_raw();

            *text_pos += Vector2::new(1f32, 0f32);
            pd.print(format!("Text Pos: {}, {}", text_pos.x, text_pos.y));
            pd.graphics.draw_text("Helo", text_pos.try_cast().unwrap());
            pd.system.draw_fps(Vector2::new(0,0));
        }
    }

    // pd.sys().api().get_button_state
    1
}

pub extern "C" fn update(pd: &Playdate) -> i32 {
    // pd.
    1
}