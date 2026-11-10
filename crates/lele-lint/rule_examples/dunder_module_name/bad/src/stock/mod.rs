#[path = "__basic__/mod.rs"]
pub mod __basic__;

#[path = "__traits__/mod.rs"]
mod traits;

pub use __basic__::structs::Item;
pub use traits::Greetable;
