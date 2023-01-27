use std::ffi::c_void;
use bitflags::bitflags;
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

bitflags! {
    pub struct PDButtons: i32 {
        const LEFT 	= (1<<0);
		const RIGHT	= (1<<1);
		const UP		= (1<<2);
		const DOWN	= (1<<3);
		const B		= (1<<4);
		const A		= (1<<5);
    }
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
pub struct PDMenuItem {
	_data: [u8; 0],
	_marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>
}

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
	pub get_seconds_since_epoch: unsafe extern "C" fn(s: *mut u32) -> u32,

	// void (*drawFPS)(int x, int y);
	pub draw_fps: unsafe extern "C" fn(x: i32, y: i32),



	// void (*setUpdateCallback)(PDCallbackFunction* update, void* userdata);
	pub set_update_callback: unsafe extern "C" fn(update: PDCallbackFunction, userdata: *const c_void),

	// void (*getButtonState)(PDButtons* current, PDButtons* pushed, PDButtons* released);
	pub get_button_state: unsafe extern "C" fn(
		current: *mut i32,
		pushed: *mut i32,
		released: *mut i32
	),

	// void (*setPeripheralsEnabled)(PDPeripherals mask);
	pub set_peripherals_enabled: unsafe extern "C" fn(mask: PDPeripherals),

	// void (*getAccelerometer)(float* outx, float* outy, float* outz);
	pub get_accelerometer: unsafe extern "C" fn(outx: *mut f32, outy: *mut f32, outz: *mut f32),



	// float (*getCrankChange)(void);
	pub get_crank_change: unsafe extern "C" fn() -> f32,

	// float (*getCrankAngle)(void);
	pub get_crank_angle: unsafe extern "C" fn() -> f32,

	// int (*isCrankDocked)(void);
	pub is_crank_docked: unsafe extern "C" fn() -> i32,

	// int (*setCrankSoundsDisabled)(int flag); // returns previous setting
	pub set_crank_sound_disabled: unsafe extern "C" fn(flag: i32) -> i32,



	// int (*getFlipped)(void);
	pub get_flipped: unsafe extern "C" fn() -> i32,

	// void (*setAutoLockDisabled)(int disable);
	pub set_auto_lock_disabled: unsafe extern "C" fn(disable: i32),



	// void (*setMenuImage)(LCDBitmap* bitmap, int xOffset);
	pub set_menu_image: unsafe extern "C" fn(disable: i32),

	// PDMenuItem* (*addMenuItem)(const char *title, PDMenuItemCallbackFunction* callback, void* userdata);
	pub add_menu_item: unsafe extern "C" fn(
		title: *const char,
		callback: PDMenuItemCallbackFunction,
		userdata: *mut c_void
	) -> *const PDMenuItem,

	// PDMenuItem* (*addCheckmarkMenuItem)(const char *title, int value, PDMenuItemCallbackFunction* callback, void* userdata);
	pub add_checkmark_menu_item: unsafe extern "C" fn(
		title: *const char,
		callback: PDMenuItemCallbackFunction,
		userdata: *mut c_void
	) -> *const PDMenuItem,

	// PDMenuItem* (*addOptionsMenuItem)(const char *title, const char** optionTitles, int optionsCount, PDMenuItemCallbackFunction* f, void* userdata);
	pub add_options_menu_item: unsafe extern "C" fn(
		title: *const char,
		option_titles: *mut *const char,
		options_count: i32,
		callback: PDMenuItemCallbackFunction,
		userdata: *mut c_void
	) -> *const PDMenuItem,

	// void (*removeAllMenuItems)(void);
	pub remove_all_menu_items: unsafe extern "C" fn(),

	// void (*removeMenuItem)(PDMenuItem *menuItem);
	pub remove_menu_item: unsafe extern "C" fn(menu_item: *const PDMenuItem),

	// int (*getMenuItemValue)(PDMenuItem *menuItem);
	pub get_menu_item_value: unsafe extern "C" fn(menu_item: *const PDMenuItem) -> i32,

	// void (*setMenuItemValue)(PDMenuItem *menuItem, int value);
	pub set_menu_item_value: unsafe extern "C" fn(menu_item: *const PDMenuItem, value: i32),

	// const char* (*getMenuItemTitle)(PDMenuItem *menuItem);
	pub get_menu_item_title: unsafe extern "C" fn(menu_item: *const PDMenuItem) -> *const char,

	// void (*setMenuItemTitle)(PDMenuItem *menuItem, const char *title);
	pub set_menu_item_title: unsafe extern "C" fn(menu_item: *const PDMenuItem, title: *const char),

	// void* (*getMenuItemUserdata)(PDMenuItem *menuItem);
	pub get_menu_item_user_data: unsafe extern "C" fn(menu_item: *const PDMenuItem) -> *mut c_void,

	// void (*setMenuItemUserdata)(PDMenuItem *menuItem, void *ud);
	pub set_menu_item_user_data: unsafe extern "C" fn(menu_item: *const PDMenuItem, *mut c_void),



	// int (*getReduceFlashing)(void);
	pub get_reduce_flashing: unsafe extern "C" fn() -> i32,


	// // 1.1
	// float (*getElapsedTime)(void);
	pub get_elapsed_time: unsafe extern "C" fn() -> f32,

	// void (*resetElapsedTime)(void);
	pub reset_elapsed_time: unsafe extern "C" fn(),


	// // 1.4
	// float (*getBatteryPercentage)(void);
	pub get_battery_percentage: unsafe extern "C" fn() -> f32,

	// float (*getBatteryVoltage)(void);
	pub get_battery_voltage: unsafe extern "C" fn() -> f32,
}