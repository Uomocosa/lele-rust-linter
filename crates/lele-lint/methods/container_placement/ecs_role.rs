use crate::rules;

pub fn ecs_role(derives: &[String]) -> Option<rules::container_placement::Role> {
    for derive in derives {
        match derive.as_str() {
            "Resource" => return Some(rules::container_placement::Role::Resource),
            "Component" => return Some(rules::container_placement::Role::Component),
            "Message" => return Some(rules::container_placement::Role::Message),
            "Event" => return Some(rules::container_placement::Role::Event),
            _ => {}
        }
    }
    None
}

// no test_usage necessary
