use std::path::Path;

use syn::spanned::Spanned;

use crate::Project;

pub fn rustfmt_one_liner(project: &Project, rel_path: &Path, stmt: &syn::Stmt) -> bool {
    let start = stmt.span().start();
    project
        .rustfmt_one_liners
        .get(&(rel_path.to_path_buf(), start.line, start.column))
        .copied()
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let source = "impl Foo { fn one(&self) -> u8 { self.0 } }\n";
        std::fs::write(dir.path().join("foo.rs"), source).unwrap();
        let mut project = Project {
            root: PathBuf::from("."),
            src_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        let file: syn::File = syn::parse_str(source).unwrap();
        let syn::Item::Impl(block) = &file.items[0] else {
            panic!("expected an impl block");
        };
        let syn::ImplItem::Fn(method) = &block.items[0] else {
            panic!("expected a method");
        };
        let stmt = &method.block.stmts[0];
        project
            .parsed_files
            .insert(PathBuf::from("foo.rs"), file.clone());
        project.fill_rustfmt_answers().unwrap();
        assert!(project.rustfmt_one_liner(Path::new("foo.rs"), stmt));
        assert!(!project.rustfmt_one_liner(Path::new("bar.rs"), stmt));
    }
}
