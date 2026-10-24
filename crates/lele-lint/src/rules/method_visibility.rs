use crate::Category;
use crate::Entry;
use crate::ExampleFile;
use crate::ModuleInfoMap;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::collections::HashSet;

pub struct MethodVisibility;

impl MethodVisibility {
    pub const NAME: &'static str = "method_visibility";
    pub const CODE: &'static str = "LELE004";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "A method file is a private module: declared with `mod`, never `pub mod` or `pub use`.",
        why: "Method bodies are reached only through the type's methods, so callers can't depend on the file layout.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod player;
",
            },
            ExampleFile {
                path: "src/player/mod.rs",
                source: r"mod player;
pub mod player_rename;

pub use player::Player;
",
            },
            ExampleFile {
                path: "src/player/player.rs",
                source: r"pub struct Player {
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
",
            },
            ExampleFile {
                path: "src/player/player_rename.rs",
                source: r"use crate::player;

pub fn rename(player: &mut player::Player, name: String) {
    player.name = name;
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
impl Rule for MethodVisibility {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(MethodVisibility)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl MethodVisibility {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(MethodVisibility));
    }
}

#[atomic_delegates]
impl MethodVisibility {
    pub(crate) fn is_actually_method_file(
        file_name: &str,
        parent_dir: &str,
        project: &Project,
    ) -> bool {
    }
    pub(crate) fn group_entries_by_parent_dir(
        entries: &[Entry],
    ) -> std::collections::BTreeMap<String, Vec<String>> {
    }
    pub(crate) fn collect_struct_names(
        file_names: &[String],
        parent_dir: &str,
        project: &Project,
    ) -> HashSet<String> {
    }
    pub(crate) fn declared_as_pub_mod(
        module_info: &ModuleInfoMap,
        parent_dir: &str,
        mod_name: &str,
    ) -> Option<std::path::PathBuf> {
    }
    pub(crate) fn reexported_in_pub_use(
        module_info: &ModuleInfoMap,
        parent_dir: &str,
        mod_name: &str,
    ) -> Option<std::path::PathBuf> {
    }
}

// no test_usage necessary
