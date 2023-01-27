use std::{ffi::{CString, NulError}, pin::Pin, mem::MaybeUninit};

use na::Vector2;

use crate::{
    api::system::{
        PlaydateSys,
        PDCallbackFunction,
        PDButtons
    }
};

pub use crate::{
    api::system::{PDLanguage as Language}
};

pub struct System(PlaydateSys);

pub type UpdateCallback<T> = extern "C" fn(userdata: &mut T) -> i32;



pub struct ButtonState {
    pub current: PDButtons,
    pub pushed: PDButtons,
    pub released: PDButtons,
}

impl System {
    pub fn api(&self) -> &PlaydateSys {
        &self.0
    }

    /*-------------PRINTING----------------*/
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

    pub fn error<T: Into<Vec<u8>>>(&self, s: T)
    {
        self.error_checked(s).unwrap()
    }

    pub fn error_checked<T: Into<Vec<u8>>>(&self, s: T) -> Result<(), NulError>
    {
        CString::new(s)
            .map(|cstring|{
                unsafe { (self.0.error)(std::mem::transmute(cstring.into_raw())) };
            })
    }


    /*---------------LANGUAGE------------------*/
    pub fn get_language(&self) -> Language {
        unsafe { (self.0.get_language)() }
    }

    /*----------------TIME---------------- */
    pub fn get_unix_time_seconds_milliseconds(&self) -> (u32, u32) {
        let mut ms: MaybeUninit<u32> = MaybeUninit::uninit();
        let seconds = unsafe { (self.0.get_seconds_since_epoch)(ms.as_mut_ptr()) };
        (seconds, unsafe { ms.assume_init() })
    }

    pub fn get_unix_time_seconds(&self) -> u32 {
        self.get_unix_time_seconds_milliseconds().0
    }

    pub fn get_unix_time_milliseconds(&self) -> u32 {
        self.get_unix_time_seconds_milliseconds().1
    }


    pub fn get_elapsed_time(&self) -> f32 {
        unsafe { (self.0.get_elapsed_time)() }
    }

    pub fn reset_elapsed_time(&self) {
        unsafe { (self.0.reset_elapsed_time)(); }
    }


    /*-----------------UTIL-------------- */
    pub fn draw_fps(&self, pos: Vector2<i32>){
        unsafe { (self.0.draw_fps)(pos.x, pos.y); }
    }


    /*--------------UPDATE CALLBACK------------------*/
    pub fn set_update_callback<T>(&self, f: UpdateCallback<T>, userdata: &mut T) {
        unsafe { (self.0.set_update_callback)(std::mem::transmute(f), std::mem::transmute(userdata)); }
    }

    /*-------------INPUT--------------------- */
    pub fn get_button_state_raw(&self) -> (i32,i32,i32) {
        unsafe {
            let mut current: i32 = 0;
            let mut pushed: i32 = 0;
            let mut released: i32 = 0;
            (self.0.get_button_state)(
                &mut current,
                &mut pushed,
                &mut released
            );
            (current, pushed, released)
        }
    }

    pub fn get_button_state(&self) -> ButtonState {
        let (current, pushed, released) = self.get_button_state_raw();
        unsafe {
            ButtonState{
                current: PDButtons::from_bits_unchecked(current),
                pushed: PDButtons::from_bits_unchecked(pushed),
                released: PDButtons::from_bits_unchecked(released)
            }
        }
    }

    pub fn get_buttons_current(&self) -> PDButtons {
        self.get_button_state().current
    }

    pub fn get_buttons_pushed(&self) -> PDButtons {
        self.get_button_state().pushed
    }

    pub fn get_buttons_released(&self) -> PDButtons {
        self.get_button_state().released
    }

    pub fn get_dpad_axes(&self) -> na::Vector2<f32> {
        let cur = self.get_buttons_current();
        na::Vector2::new(
            if cur.contains(PDButtons::LEFT) { -1f32 } else { 0f32 }
            + if cur.contains(PDButtons::RIGHT) { 1f32 } else { 0f32 },
            if cur.contains(PDButtons::UP) { 1f32 } else { 0f32 }
            + if cur.contains(PDButtons::DOWN) { -1f32 } else { 0f32 },
        )
    }

    pub fn get_dpad_axes_normalized(&self) -> na::Vector2<f32> {
        self.get_dpad_axes().normalize()
    }
}