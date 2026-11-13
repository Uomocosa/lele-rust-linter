use std::collections::BTreeMap;

use syn::spanned::Spanned;

use crate::Error;
use crate::RustfmtKey;

pub fn rustfmt_answers(
    output: &str,
    keys: &[RustfmtKey],
    max_width: usize,
) -> Result<BTreeMap<RustfmtKey, bool>, Error> {
    let file = syn::parse_file(output).map_err(Error::RustfmtOutput)?;
    let lines: Vec<&str> = output.lines().collect();
    let mut one_line: BTreeMap<String, bool> = BTreeMap::new();
    for item in &file.items {
        let syn::Item::Impl(imp) = item else {
            continue;
        };
        for inner in &imp.items {
            let syn::ImplItem::Fn(method) = inner else {
                continue;
            };
            let answer = method.block.stmts.first().is_some_and(|stmt| {
                let span = stmt.span();
                span.start().line == span.end().line
                    && lines
                        .get(span.start().line.saturating_sub(1))
                        .is_some_and(|line| line.chars().count() <= max_width)
            });
            one_line.insert(method.sig.ident.to_string(), answer);
        }
    }
    keys.iter()
        .enumerate()
        .map(|(index, key)| {
            one_line
                .get(&format!("__lele_{index}"))
                .map(|answer| (key.clone(), *answer))
                .ok_or(Error::RustfmtMissing)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Error;
    use crate::RustfmtKey;

    #[test]
    fn test_usage() {
        let key: RustfmtKey = (PathBuf::from("a.rs"), 1, 0);
        let output = "impl __Lele {\n    fn __lele_0() {\n        g()\n    }\n    fn __lele_1() {\n        h\n            .k()\n    }\n}\n";
        let answers =
            super::rustfmt_answers(output, &[key.clone(), (PathBuf::from("b.rs"), 1, 0)], 100)
                .unwrap();
        assert_eq!(answers.get(&key), Some(&true));
        assert_eq!(answers.get(&(PathBuf::from("b.rs"), 1, 0)), Some(&false));
        assert!(matches!(
            super::rustfmt_answers("impl __Lele {}\n", std::slice::from_ref(&key), 100),
            Err(Error::RustfmtMissing)
        ));
        let long = format!("        g(\"{}\")", "a".repeat(110));
        let wide = format!("impl __Lele {{\n    fn __lele_0() {{\n{long}\n    }}\n}}\n");
        let narrow = super::rustfmt_answers(&wide, std::slice::from_ref(&key), 100).unwrap();
        assert_eq!(narrow.get(&key), Some(&false));
        let roomy = super::rustfmt_answers(&wide, std::slice::from_ref(&key), 200).unwrap();
        assert_eq!(roomy.get(&key), Some(&true));
    }
}
