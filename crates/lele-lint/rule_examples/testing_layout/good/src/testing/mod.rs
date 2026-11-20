#[path = "__fixtures__/mod.rs"]
pub mod fixtures;

#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::enums::Occupancy;
pub use fixtures::lobby_row::LobbyRow;
