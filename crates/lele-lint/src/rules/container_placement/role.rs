#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Role {
    Resource,
    Component,
    Message,
    Event,
    Newtype,
    Struct,
    Enum,
    TypeAlias,
    Const,
}

#[rustfmt::skip]
impl Role {
    pub(crate) const ALL: [Role; 9] = [
        Role::Resource,
        Role::Component,
        Role::Message,
        Role::Event,
        Role::Newtype,
        Role::Struct,
        Role::Enum,
        Role::TypeAlias,
        Role::Const,
    ];
    pub(crate) fn file(self) -> &'static str {
        match self {
            Role::Resource => "resources.rs",
            Role::Component => "components.rs",
            Role::Message => "messages.rs",
            Role::Event => "events.rs",
            Role::Newtype => "newtypes.rs",
            Role::Struct => "structs.rs",
            Role::Enum => "enums.rs",
            Role::TypeAlias => "type_aliases.rs",
            Role::Const => "constants.rs",
        }
    }
}

// no test_usage necessary
