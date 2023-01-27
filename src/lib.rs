use pdrs::alloc::set_pd_api;
use std::{ffi::{c_void}, ptr::null};
use std::pin::Pin;

pub(crate) mod api;
pub(crate) mod pdrs;
use api::{
    PlaydateAPI,
    event::{SystemEvent}, graphics::{LCDColor, LCDSolidColor},
};

use pdrs::playdate::Playdate;

#[repr(C)]
pub struct UserData {
    pub pd: &'static Playdate,
    pub x: f32, pub y: f32,
}

// static mut PD: Option<Box<Playdate>> = None;

#[no_mangle]
pub unsafe extern "C" fn eventHandler(
    pd: &'static Playdate,
    // pd_raw: Playdate,
    ev: SystemEvent,
    arg: u32
) -> i32 {
    match ev {
        SystemEvent::Init => {
            {
                // let pd = Playdate::from(pd_raw);
                // pd.init_allocator();
                // PD = Some(Box::new(pd.clone()));
            }
            pd.init_allocator();
            // PD = Some(Box::new(pd_ref));
            

            // let pd: &Box<Playdate> = PD.as_ref().unwrap();
            
            // pd.sys().set_update_callback::<Playdate>(update, &*pd);
            static mut USERDATA: Option<UserData> = None;
            USERDATA = Some(UserData {
                pd: pd,
                x: 0f32, y: 0f32
            });
            pd.sys().set_update_callback(update0, USERDATA.as_ref().unwrap());
        }
        _ => {
            // let pd = PD.as_ref().unwrap();
        }
    };
    0
}

pub extern "C" fn update0(userdata: &UserData) -> i32 {
    let &UserData {pd, x, y} = userdata;
    pd.print("Hello");
    // pd.sys().api().get_button_state
    1
}

pub extern "C" fn update(pd: &Playdate) -> i32 {
    // pd.
    1
}