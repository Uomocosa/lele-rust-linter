mod player;
#[path = "__bevy_systems__/mod.rs"]
pub mod bevy_systems;

#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::Event;
pub use player::Player;
