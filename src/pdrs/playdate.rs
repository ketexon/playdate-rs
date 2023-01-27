use crate::api::{
    PlaydateAPI,
    system::PlaydateSys
};
use super::alloc;
use super::system::System;

#[repr(C)]
pub struct Playdate(PlaydateAPI);

impl Playdate {
    pub fn init_allocator(&self) {
        unsafe { alloc::set_pd_api(&self.0); }
    }

    pub fn api(&self) -> &PlaydateAPI {
        &self.0
    }

    pub fn sys(&self) -> System {
        unsafe { System::from(self.0.system.as_ref().expect("System is NULL")) }
    }

    // Redefinitions of common functions
    pub fn print<T: Into<Vec<u8>>>(&self, s: T) {
        self.sys().print(s);
    }
}