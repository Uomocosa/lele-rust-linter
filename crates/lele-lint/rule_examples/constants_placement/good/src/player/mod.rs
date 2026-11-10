mod health;
#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::constants::MAX_HEALTH;
pub use health::Health;
