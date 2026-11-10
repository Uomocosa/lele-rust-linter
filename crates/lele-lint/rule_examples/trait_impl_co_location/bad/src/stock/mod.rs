pub mod clock;
pub mod unix_time;
#[path = "__basic__/mod.rs"]
mod basic;
#[path = "__traits__/mod.rs"]
pub mod traits;

pub use basic::structs::Dial;
