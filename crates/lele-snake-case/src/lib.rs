#[path = "__basic__/mod.rs"]
mod basic;
mod delegate_file;
mod snake_case_walk;
#[path = "__traits__/mod.rs"]
pub mod traits;
pub use delegate_file::DelegateFile;
pub use snake_case_walk::SnakeCaseWalk;
