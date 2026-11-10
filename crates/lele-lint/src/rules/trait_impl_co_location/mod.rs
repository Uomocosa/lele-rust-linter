#[path = "__basic__/mod.rs"]
mod basic;
mod impl_visitor;
mod trait_impl_co_location;
mod type_visitor;
pub(crate) use basic::structs::ImplHit;
pub(crate) use basic::type_aliases::DefinitionMap;
pub(crate) use impl_visitor::ImplVisitor;
pub use trait_impl_co_location::TraitImplCoLocation;
pub(crate) use type_visitor::TypeVisitor;
