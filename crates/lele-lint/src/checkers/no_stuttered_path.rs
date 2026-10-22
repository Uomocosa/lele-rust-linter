use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct NoStutteredPath;

impl NoStutteredPath {
    pub const NAME: &'static str = "no_stuttered_path";
    pub const CODE: &'static str = "LELE011";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "No `meters::Meters` paths for crate-root modules: import the type once and write `Meters`.",
        why: "The module and type say the same word twice; the type name alone is already unambiguous.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod meters;
pub use meters::Meters;
mod track;
pub use track::Track;
",
            },
            ExampleFile {
                path: "src/meters.rs",
                source: r"use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Meters(pub u32);

#[rustfmt::skip]
impl Meters {
    pub fn doubled(&self) -> u32 { **self * 2 }
}

#[cfg(test)]
mod tests {
    use crate::Meters;

    #[test]
    fn test_usage() {
        assert_eq!(Meters(4).doubled(), 8);
    }
}
",
            },
            ExampleFile {
                path: "src/track.rs",
                source: r#"use crate::meters;

pub struct Track {
    pub name: String,
    pub length: meters::Meters,
}

#[rustfmt::skip]
impl Track {
    pub fn laps_for(&self, distance: u32) -> u32 { distance / *self.length }
}

#[cfg(test)]
mod tests {
    use crate::meters;
    use crate::Track;

    #[test]
    fn test_usage() {
        let track = Track { name: "oval".to_string(), length: meters::Meters(400) };
        assert_eq!(track.laps_for(800), 2);
    }
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod meters;
pub use meters::Meters;
mod track;
pub use track::Track;
",
            },
            ExampleFile {
                path: "src/meters.rs",
                source: r"use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Meters(pub u32);

#[rustfmt::skip]
impl Meters {
    pub fn doubled(&self) -> u32 { **self * 2 }
}

#[cfg(test)]
mod tests {
    use crate::Meters;

    #[test]
    fn test_usage() {
        assert_eq!(Meters(4).doubled(), 8);
    }
}
",
            },
            ExampleFile {
                path: "src/track.rs",
                source: r#"use crate::Meters;

pub struct Track {
    pub name: String,
    pub length: Meters,
}

#[rustfmt::skip]
impl Track {
    pub fn laps_for(&self, distance: u32) -> u32 { distance / *self.length }
}

#[cfg(test)]
mod tests {
    use crate::Meters;
    use crate::Track;

    #[test]
    fn test_usage() {
        let track = Track { name: "oval".to_string(), length: Meters(400) };
        assert_eq!(track.laps_for(800), 2);
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Checker for NoStutteredPath {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoStutteredPath)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoStutteredPath {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(NoStutteredPath));
    }
}

// no test_usage necessary
