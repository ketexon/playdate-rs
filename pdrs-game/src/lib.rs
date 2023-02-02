extern crate pdrs_core;
extern crate pdrs_game_macro;
use pdrs_core::Playdate;
use pdrs_core::event::Event;

pub use pdrs_game_macro::game;

pub trait Game {
    fn new(pd: &'static Playdate) -> Self;
    fn handle_event(&mut self, event: Event);
    fn update(&mut self);
}