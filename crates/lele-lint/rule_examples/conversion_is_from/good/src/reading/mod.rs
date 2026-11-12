#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::newtypes::SensorName;
mod reading;

pub use reading::Reading;
