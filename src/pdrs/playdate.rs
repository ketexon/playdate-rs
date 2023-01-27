use crate::api::file;
use crate::api::{
    PlaydateAPI,
    system::PlaydateSys
};
use super::alloc;
use super::system::System;
use super::graphics::Graphics;

#[repr(C)]
pub struct Playdate {
    pub system: &'static System,
    pub file: *const file::PlaydateFile,
    pub graphics: &'static Graphics
}

impl Playdate {
    pub fn init_allocator(&self) {
        unsafe { alloc::set_pd_api(self.api()); }
    }

    pub fn api(&self) -> &PlaydateAPI {
        unsafe { std::mem::transmute(self) }
    }

    /*-------------Redefinitions of common functions-------------*/
    pub fn print<T: Into<Vec<u8>>>(&self, s: T) {
        self.system.print(s);
    }
}