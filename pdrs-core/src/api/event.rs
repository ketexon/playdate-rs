// typedef enum
// {
// 	kEventInit,
// 	kEventInitLua,
// 	kEventLock,
// 	kEventUnlock,
// 	kEventPause,
// 	kEventResume,
// 	kEventTerminate,
// 	kEventKeyPressed, // arg is keycode
// 	kEventKeyReleased,
// 	kEventLowPower
// } PDSystemEvent;

#[repr(C)]
pub enum SystemEvent {
    Init,
	InitLua,
	Lock,
	Unlock,
	Pause,
	Resume,
	Terminate,
    // arg is keycode
	KeyPressed, 
	KeyReleased,
	LowPower,
}