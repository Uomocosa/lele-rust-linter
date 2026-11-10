#[path = "__basic__/mod.rs"]
pub mod basic;

#[path = "__traits__/mod.rs"]
mod traits;

pub use basic::enums::MountKind;
pub use traits::Greetable;
