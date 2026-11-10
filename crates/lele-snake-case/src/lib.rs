#[path = "__basic__/mod.rs"]
mod basic;
mod snake_case_walk;
#[path = "__traits__/mod.rs"]
pub mod traits;
pub use snake_case_walk::SnakeCaseWalk;
