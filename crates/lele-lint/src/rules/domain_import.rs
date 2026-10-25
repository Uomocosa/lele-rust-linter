use crate::Category;
use crate::DunderPath;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use syn::spanned::Spanned;

pub struct DomainImport;

impl DomainImport {
    pub const NAME: &'static str = "domain_import";
    pub const CODE: &'static str = "LELE016";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "Import the domain, not the item: `use crate::stock;` then `stock::Item`.",
        why: "The domain prefix at every use site says where a name comes from, so names never need to be unique crate-wide.",
        bad: rule_examples!("domain_import/bad"),
        good: rule_examples!("domain_import/good"),
    };
}

#[rustfmt::skip]
impl Rule for DomainImport {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(DomainImport)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl DomainImport {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(DomainImport));
    }
    pub(crate) fn is_pub_use(item_use: &syn::ItemUse) -> bool {
        matches!(item_use.vis, syn::Visibility::Public(_))
    }
    pub(crate) fn collect_use_segments(tree: &syn::UseTree) -> Vec<String> {
        match tree {
            syn::UseTree::Path(p) => {
                let mut segs = vec![p.ident.to_string()];
                segs.extend(Self::collect_use_segments(&p.tree));
                segs
            }
            syn::UseTree::Name(n) => vec![n.ident.to_string()],
            syn::UseTree::Rename(r) => vec![r.ident.to_string()],
            syn::UseTree::Glob(_) => Vec::new(),
            syn::UseTree::Group(_) => Vec::new(),
        }
    }
    pub(crate) fn find_use_line(item_use: &syn::ItemUse) -> usize {
        item_use.span().start().line
    }
}

#[atomic_delegates]
impl DomainImport {
    pub(crate) fn check_import(
        item_use: &syn::ItemUse,
        dunder_paths: &[DunderPath],
    ) -> Option<String> {
    }
}

// no test_usage necessary
