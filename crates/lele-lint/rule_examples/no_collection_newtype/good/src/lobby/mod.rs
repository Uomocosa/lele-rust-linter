#[path = "__basic__/mod.rs"]
mod basic;

mod deck;

pub use basic::newtypes::Room;
pub use basic::resources::PeerNames;
pub use basic::structs::Card;
pub use deck::Deck;
