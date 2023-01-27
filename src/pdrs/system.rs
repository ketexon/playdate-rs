use std::{ffi::{CString, NulError}, pin::Pin, mem::MaybeUninit};

use crate::{
    api::system::{
        PlaydateSys, 
        PDCallbackFunction,
        PDButtons
    }
};

pub struct System<'a>(&'a PlaydateSys);

pub type UpdateCallback<T> = extern "C" fn(userdata: &T) -> i32;

pub struct ButtonState {
    pub down: PDButtons,
    pub pushed: PDButtons,
    pub released: PDButtons,
}

impl<'a> System<'a> {
    pub fn api(&self) -> &'a PlaydateSys {
        self.0
    }

    pub fn print<T: Into<Vec<u8>>>(&self, s: T)
    {
        self.print_checked(s).unwrap()
    }

    pub fn print_checked<T: Into<Vec<u8>>>(&self, s: T) -> Result<(), NulError>
    {
        CString::new(s)
            .map(|cstring|{
                unsafe { (self.0.log_to_console)(std::mem::transmute(cstring.into_raw())) };
            })
    }

    pub fn set_update_callback<T>(&self, f: UpdateCallback<T>, userdata: &T) {
        unsafe { (self.0.set_update_callback)(std::mem::transmute(f), std::mem::transmute(userdata)); }
    }

    pub fn get_button_state(&self) -> ButtonState {
        unsafe {
            let mut button_state: ButtonState = ButtonState {
                down: PDButtons::ButtonA,
                pushed: PDButtons::ButtonA,
                released: PDButtons::ButtonA
            };
            
            let mut button_down: MaybeUninit<PDButtons> = MaybeUninit::uninit();
            (self.0.get_button_state)(
                Some(button_down.as_mut_ptr()),
                Some(&mut button_state.pushed),
                Some(&mut button_state.released)
            );
            button_state
        }
    }
}

impl<'a> From<&'a PlaydateSys> for System<'a> {
    fn from(pd_api: &'a PlaydateSys) -> Self {
        Self(pd_api)
    }
}

impl<'a> From<*const PlaydateSys> for System<'a> {
    fn from(pd_api_ptr: *const PlaydateSys) -> Self {
        Self(unsafe { 
            pd_api_ptr.as_ref().expect("pd_api_raw is null"
        )})
    }
}