pub use crate::api::event::{
    SystemEvent
};

pub enum Event {
    Init,
	InitLua,
	Lock,
	Unlock,
	Pause,
	Resume,
	Terminate,
	KeyPressed(u32), 
	KeyReleased(u32),
	LowPower,
}

impl Event {
    pub fn from_system_event(value: SystemEvent, arg: u32) -> Self {
        match value {
            SystemEvent::Init => Self::Init,
            SystemEvent::InitLua => Self::InitLua,
            SystemEvent::Lock => Self::Lock,
            SystemEvent::Unlock => Self::Unlock,
            SystemEvent::Pause => Self::Pause,
            SystemEvent::Resume => Self::Resume,
            SystemEvent::Terminate => Self::Terminate,
            SystemEvent::KeyPressed => Self::KeyPressed(arg),
            SystemEvent::KeyReleased => Self::KeyReleased(arg),
            SystemEvent::LowPower => Self::LowPower
        }
    }
}