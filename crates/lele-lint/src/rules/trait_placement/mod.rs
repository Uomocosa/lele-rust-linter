#[path = "__basic__/mod.rs"]
mod basic;
mod trait_placement;
pub(crate) mod trait_visitor;
pub(crate) use basic::structs::TraitHit;
pub use trait_placement::TraitPlacement;
pub(crate) use trait_visitor::TraitVisitor;
