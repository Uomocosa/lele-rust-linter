mod diver;

#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::constants::MAX_DEPTH;
pub use diver::Diver;
