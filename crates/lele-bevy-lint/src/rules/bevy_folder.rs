use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use syn::visit::Visit;

use crate::scan;

pub struct BevyFolder;

impl BevyFolder {
    pub const NAME: &'static str = "bevy_folder";
    pub const CODE: &'static str = "LELEBEVY002";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A `pub fn` registered with `app.add_systems()` whose parameters read Bevy system types must live in the domain's `bevy_systems/` folder.",
        why: "Systems are the domain's Bevy edge; keeping them in one folder makes the plugin surface readable.",
        bad: &[ExampleFile {
            path: "src/enemy/tick_enemies.rs",
            source: r"pub struct Query;

pub fn tick_enemies(_q: Query) {}

pub fn register_enemies() {
    let app = App;
    app.add_systems(Update, tick_enemies);
}
",
        }],
        good: &[
            ExampleFile {
                path: "src/enemy/mod.rs",
                source: r"pub mod bevy_systems;
",
            },
            ExampleFile {
                path: "src/enemy/bevy_systems/mod.rs",
                source: r"pub mod tick_enemies;
pub use tick_enemies::tick_enemies;
",
            },
            ExampleFile {
                path: "src/enemy/bevy_systems/tick_enemies.rs",
                source: r"pub struct Query;

pub fn tick_enemies(_q: Query) {}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for BevyFolder {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyFolder)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyFolder {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
    pub(crate) fn is_bevy_system_param(arg: &syn::FnArg) -> bool {
        let syn::FnArg::Typed(pat_type) = arg else { return false; };
        Self::last_path_ident(&pat_type.ty).is_some_and(|ident| SYSTEM_PARAM_NAMES.contains(&ident.as_str()))
    }
    pub(crate) fn last_path_ident(ty: &syn::Type) -> Option<String> {
        match ty { syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()), syn::Type::Reference(r) => Self::last_path_ident(&r.elem), _ => None, }
    }
    pub(crate) fn registered_systems(file: &syn::File) -> Vec<String> {
        let mut found = Vec::new();
        { let mut collector = scan::RegisteredCollector(&mut found); collector.visit_file(file); }
        found
    }
}

const SYSTEM_PARAM_NAMES: [&str; 6] = [
    "Res",
    "ResMut",
    "Query",
    "Commands",
    "MessageWriter",
    "MessageReader",
];

// no test_usage necessary
