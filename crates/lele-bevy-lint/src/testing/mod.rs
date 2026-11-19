#![cfg(test)]

#[path = "__fixtures__/mod.rs"]
pub(crate) mod fixtures;
#[path = "__queries__/mod.rs"]
pub(crate) mod queries;

pub(crate) use fixtures::bevy_project::bevy_project;
pub(crate) use fixtures::macro_of::macro_of;
pub(crate) use fixtures::method_call::method_call;
pub(crate) use fixtures::no_visuals::no_visuals;
pub(crate) use fixtures::preview::preview;
pub(crate) use fixtures::preview_source::preview_source;
pub(crate) use fixtures::preview_violation::preview_violation;
pub(crate) use fixtures::project_with::project_with;
pub(crate) use queries::call_graph::call_graph;
pub(crate) use queries::harness_call::harness_call;
pub(crate) use queries::harness_run_imported::harness_run_imported;
pub(crate) use queries::messages_of::messages_of;
pub(crate) use queries::rule_messages::rule_messages;
pub(crate) use queries::rule_violations::rule_violations;
pub(crate) use queries::violations_of::violations_of;
