use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::collections::BTreeSet;
use std::path::Path;

pub struct NoFreeFunctions;

impl NoFreeFunctions {
    pub const NAME: &'static str = "no_free_functions";
    pub const CODE: &'static str = "LELE034";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "A `pub`/`pub(crate)` top-level `fn` in `src/` must be a method of a type: its owner is the first argument's type or the return type; body in `methods/<type>/<method>.rs`. `bevy_systems/`, `__basic__/`, `main.rs`, `build.rs` and `[[lele.lint.entry_points]]` are exempt.",
        why: "`src/` lists types; the `#[atomic_delegates]` block of each type says what it can do. A free function hides which type it works on, so nobody finds it from the type.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod player;
",
            },
            ExampleFile {
                path: "src/player/mod.rs",
                source: r"mod heal;
mod player;

pub use heal::heal;
pub use player::Player;
",
            },
            ExampleFile {
                path: "src/player/player.rs",
                source: r"pub struct Player {
    pub name: String,
    pub health: u32,
}
",
            },
            ExampleFile {
                path: "src/player/heal.rs",
                source: r"use crate::player;

pub fn heal(player: &mut player::Player, amount: u32) {
    let healed = player.health.saturating_add(amount);
    player.health = healed.min(100);
}

#[cfg(test)]
mod tests {
    use crate::player;

    #[test]
    fn test_usage() {
        let mut player = player::Player { name: String::new(), health: 10 };
        player::heal(&mut player, 5);
        assert_eq!(player.health, 15);
    }
}
",
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r#"pub mod player;

#[path = "../methods/mod.rs"]
pub mod methods;
"#,
            },
            ExampleFile {
                path: "src/player/mod.rs",
                source: r"mod player;

pub use player::Player;
",
            },
            ExampleFile {
                path: "src/player/player.rs",
                source: r"use atomic_delegate_macros::atomic_delegates;

pub struct Player {
    pub name: String,
    pub health: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: String::new(),
            health: 100,
        }
    }
}

#[atomic_delegates]
impl Player {
    pub fn heal(&mut self, amount: u32) {}
}

#[cfg(test)]
mod tests {
    use crate::player;

    #[test]
    fn test_usage() {
        let mut player = player::Player::default();
        player.heal(5);
        assert_eq!(player.health, 100);
    }
}
",
            },
            ExampleFile {
                path: "methods/mod.rs",
                source: r"pub mod player;
",
            },
            ExampleFile {
                path: "methods/player/mod.rs",
                source: r"mod heal;
pub use heal::heal;
",
            },
            ExampleFile {
                path: "methods/player/heal.rs",
                source: r"use crate::player;

pub fn heal(player: &mut player::Player, amount: u32) {
    let healed = player.health.saturating_add(amount);
    player.health = healed.min(100);
}

#[cfg(test)]
mod tests {
    use super::heal;
    use crate::player::Player;

    #[test]
    fn test_usage() {
        let mut player = Player {
            name: String::new(),
            health: 10,
        };
        heal(&mut player, 5);
        assert_eq!(player.health, 15);
    }
}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for NoFreeFunctions {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoFreeFunctions)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoFreeFunctions {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoFreeFunctions));
    }
    pub(crate) fn local_type_names(project: &Project) -> BTreeSet<String> {
        project
            .parsed_files
            .values()
            .flat_map(|file| file.items.iter())
            .filter_map(|item| match item {
                syn::Item::Struct(item) => Some(item.ident.to_string()),
                syn::Item::Enum(item) => Some(item.ident.to_string()),
                syn::Item::Trait(item) => Some(item.ident.to_string()),
                _ => None,
            })
            .collect()
    }
}

#[atomic_delegates]
impl NoFreeFunctions {
    pub(crate) fn is_exempt_file(rel_path: &Path, project: &Project) -> bool {}
    pub(crate) fn owner_hint(sig: &syn::Signature, local_types: &BTreeSet<String>) -> String {}
}

// no test_usage necessary
