use std::ffi::c_void;
/*
typedef enum
{
	kButtonLeft		= (1<<0),
	kButtonRight	= (1<<1),
	kButtonUp		= (1<<2),
	kButtonDown		= (1<<3),
	kButtonB		= (1<<4),
	kButtonA		= (1<<5)
} PDButtons;
*/

#[repr(C)]
pub enum PDButtons {
    ButtonLeft		= (1<<0),
	ButtonRight	    = (1<<1),
    ButtonUp		= (1<<2),
	ButtonDown		= (1<<3),
	ButtonB		    = (1<<4),
	ButtonA		    = (1<<5),
}

/*
typedef enum
{
	kPDLanguageEnglish,
	kPDLanguageJapanese,
	kPDLanguageUnknown,
} PDLanguage;
*/

#[repr(C)]
pub enum PDLanguage {
    PDLanguageEnglish,
	PDLanguageJapanese,
	PDLanguageUnknown,
}

/* 
typedef struct PDMenuItem PDMenuItem;
*/

#[repr(C)]
pub struct PDMenuItem;

/*
typedef enum
{
	kNone = 0,
	kAccelerometer	= (1<<0),
	// ...
	kAllPeripherals = 0xffff
} PDPeripherals;
*/

#[repr(C)]
pub enum PDPeripherals {
    None            = 0,
	Accelerometer	= (1<<0),
	// ...
	AllPeripherals  = 0xffff
}


/*

typedef int PDCallbackFunction(void* userdata); // return 0 when done
typedef void PDMenuItemCallbackFunction(void* userdata); // return 0 when done
*/

pub type PDCallbackFunction = extern "C" fn(userdata: *const c_void) -> i32;
pub type PDMenuItemCallbackFunction = extern "C" fn(userdata: *mut c_void);

/*
struct playdate_sys
{
	void* (*realloc)(void* ptr, size_t size); // ptr = NULL -> malloc, size = 0 -> free
	int (*formatString)(char **ret, const char *fmt, ...);
	void (*logToConsole)(const char* fmt, ...);
	void (*error)(const char* fmt, ...);
	PDLanguage (*getLanguage)(void);
	unsigned int (*getCurrentTimeMilliseconds)(void);
	unsigned int (*getSecondsSinceEpoch)(unsigned int *milliseconds);
	void (*drawFPS)(int x, int y);

	void (*setUpdateCallback)(PDCallbackFunction* update, void* userdata);
	void (*getButtonState)(PDButtons* current, PDButtons* pushed, PDButtons* released);
	void (*setPeripheralsEnabled)(PDPeripherals mask);
	void (*getAccelerometer)(float* outx, float* outy, float* outz);

	float (*getCrankChange)(void);
	float (*getCrankAngle)(void);
	int (*isCrankDocked)(void);
	int (*setCrankSoundsDisabled)(int flag); // returns previous setting

	int (*getFlipped)(void);
	void (*setAutoLockDisabled)(int disable);

	void (*setMenuImage)(LCDBitmap* bitmap, int xOffset);
	PDMenuItem* (*addMenuItem)(const char *title, PDMenuItemCallbackFunction* callback, void* userdata);
	PDMenuItem* (*addCheckmarkMenuItem)(const char *title, int value, PDMenuItemCallbackFunction* callback, void* userdata);
	PDMenuItem* (*addOptionsMenuItem)(const char *title, const char** optionTitles, int optionsCount, PDMenuItemCallbackFunction* f, void* userdata);
	void (*removeAllMenuItems)(void);
	void (*removeMenuItem)(PDMenuItem *menuItem);
	int (*getMenuItemValue)(PDMenuItem *menuItem);
	void (*setMenuItemValue)(PDMenuItem *menuItem, int value);
	const char* (*getMenuItemTitle)(PDMenuItem *menuItem);
	void (*setMenuItemTitle)(PDMenuItem *menuItem, const char *title);
	void* (*getMenuItemUserdata)(PDMenuItem *menuItem);
	void (*setMenuItemUserdata)(PDMenuItem *menuItem, void *ud);
	
	int (*getReduceFlashing)(void);
	
	// 1.1
	float (*getElapsedTime)(void);
	void (*resetElapsedTime)(void);

	// 1.4
	float (*getBatteryPercentage)(void);
	float (*getBatteryVoltage)(void);
};
*/

#[repr(C)]
pub struct PlaydateSys {
    // void* (*realloc)(void* ptr, size_t size); // ptr = NULL -> malloc, size = 0 -> free
    pub realloc: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void,

    // int (*formatString)(char **ret, const char *fmt, ...);
    pub format_string: unsafe extern "C" fn(*mut *mut char, *const char, ...) -> i32,

    // void (*logToConsole)(const char* fmt, ...);
    pub log_to_console: unsafe extern "C" fn(*const char, ...),

	// void (*error)(const char* fmt, ...);
	pub error: unsafe extern "C" fn(*const char, ...),

	// PDLanguage (*getLanguage)(void);
	pub get_language: unsafe extern "C" fn() -> PDLanguage,

	// unsigned int (*getCurrentTimeMilliseconds)(void);
	pub get_current_time_milliseconds: unsafe extern "C" fn() -> i32,

	// unsigned int (*getSecondsSinceEpoch)(unsigned int *milliseconds);
	pub get_seconds_since_epoch: unsafe extern "C" fn(*mut u32) -> u32,

	// void (*drawFPS)(int x, int y);
	pub draw_fps: unsafe extern "C" fn(i32, i32),



	// void (*setUpdateCallback)(PDCallbackFunction* update, void* userdata);
	pub set_update_callback: unsafe extern "C" fn(PDCallbackFunction, *const c_void),

	// void (*getButtonState)(PDButtons* current, PDButtons* pushed, PDButtons* released);
	// void (*setPeripheralsEnabled)(PDPeripherals mask);
	// void (*getAccelerometer)(float* outx, float* outy, float* outz);
}