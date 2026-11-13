use std::path::Path;

use syn::spanned::Spanned;

use crate::Error;
use crate::Origin;
use crate::Project;
use crate::RustfmtKey;

pub fn collect_rustfmt_candidates(project: &Project) -> Result<Vec<(RustfmtKey, String)>, Error> {
    let mut candidates: Vec<(RustfmtKey, String)> = Vec::new();
    let mut push = |rel: &Path, source: &str, block: &syn::Block| {
        if let [stmt] = block.stmts.as_slice() {
            let start = stmt.span().start();
            candidates.push((
                (rel.to_path_buf(), start.line, start.column),
                Project::statement_text(source, stmt),
            ));
        }
    };
    for (rel, file) in &project.parsed_files {
        let source = std::fs::read_to_string(project.absolute_src_path(rel))?;
        for item in &file.items {
            let syn::Item::Impl(imp) = item else {
                continue;
            };
            if imp.trait_.is_some() {
                continue;
            }
            for inner in &imp.items {
                if let syn::ImplItem::Fn(method) = inner {
                    push(rel, &source, &method.block);
                }
            }
        }
    }
    for (rel, file) in &project.methods_parsed_files {
        let source = std::fs::read_to_string(project.absolute_path(Origin::Methods, rel))?;
        for item in &file.items {
            if let syn::Item::Fn(func) = item {
                push(rel, &source, &func.block);
            }
        }
    }
    candidates.sort_by(|(a, _), (b, _)| a.cmp(b));
    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("methods/foo")).unwrap();
        std::fs::write(
            dir.path().join("foo.rs"),
            "impl Foo { fn a(&self) -> u8 { self.0 } fn b(&self) { let x = 1; } }\nimpl Tr for Foo { fn c(&self) -> u8 { self.0 } }\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("methods/foo/d.rs"),
            "pub fn d(f: &Foo) -> u8 { f.0 }\n",
        )
        .unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().to_path_buf(),
            methods_dir: Some(dir.path().join("methods")),
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str("impl Foo { fn a(&self) -> u8 { self.0 } fn b(&self) { let x = 1; } }\nimpl Tr for Foo { fn c(&self) -> u8 { self.0 } }\n").unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("foo/d.rs"),
            syn::parse_str("pub fn d(f: &Foo) -> u8 { f.0 }\n").unwrap(),
        );
        let candidates = project.collect_rustfmt_candidates().unwrap();
        assert_eq!(candidates.len(), 3);
        assert!(candidates.iter().any(|(_, text)| text == "f.0"));
        assert!(candidates.iter().any(|(_, text)| text == "self.0"));
    }
}
