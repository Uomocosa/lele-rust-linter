pub mod player;
pub mod player_id;
pub use player_id::PlayerId;
pub mod settings;

#[path = "../__methods__/mod.rs"]
mod methods;
