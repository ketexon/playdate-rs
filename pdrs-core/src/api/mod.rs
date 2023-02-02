pub mod display;
pub mod event;
pub mod file;
pub mod graphics;
pub mod json;
pub mod lua;
pub mod scoreboards;
pub mod sound;
pub mod sprite;
pub mod system;

/*
struct PlaydateAPI
{
	const struct playdate_sys* system;
	const struct playdate_file* file;
	const struct playdate_graphics* graphics;
	const struct playdate_sprite* sprite;
	const struct playdate_display* display;
	const struct playdate_sound* sound;
	const struct playdate_lua* lua;
	const struct playdate_json* json;
	const struct playdate_scoreboards* scoreboards;
};
*/

#[repr(C)]
pub struct PlaydateAPI {
    pub system: *const system::PlaydateSys,
    pub file: *const file::PlaydateFile,
    pub graphics: *const graphics::PlaydateGraphics,
    pub sprite: *const sprite::PlaydateSprite,
    pub display: *const display::PlaydateDisplay,
    pub sound: *const sound::PlaydateSound,
    pub lua: *const lua::PlaydateLua,
    pub json: *const json::PlaydateJson,
    pub scoreboards: *const scoreboards::PlaydateScoreboards,
}
