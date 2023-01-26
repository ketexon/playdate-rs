use std::{ffi::{c_void}, ptr::null};

pub mod event;
pub mod sys;
pub mod file;

pub use event::SystemEvent;

#[repr(C)]
pub struct PlaydateAPI {
    pub sys: *const sys::PlaydateSys,
    pub file: *const file::PlaydateFile,
}

#[no_mangle]
pub unsafe extern "C" fn eventHandler(
    pd: *const PlaydateAPI,
    ev: SystemEvent,
    arg: u32
) -> i32 {
    let pd = pd.as_ref().unwrap();
    let sys = pd.sys.as_ref().unwrap();
    match(ev){
        SystemEvent::Init => {
            const s: &'static [u8] = b"Hello from Rust!\0";
            (sys.log_to_console)(
                std::mem::transmute(s.as_ptr())
            );

            (sys.set_update_callback)(
                update, null()
            );
        }
        _ => {}
    };
    0
}

pub extern "C" fn update(userdata: *const c_void) -> i32 {
    1
}