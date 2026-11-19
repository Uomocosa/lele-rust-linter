use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn from_fn_body(body: &syn::Block, is_run_imported: bool) -> scan::HarnessCall {
    let mut scanner = scan::HarnessScanner {
        is_run_imported,
        is_called: false,
        is_checked: false,
    };
    scanner.visit_block(body);
    match (scanner.is_called, scanner.is_checked) {
        (false, _) => scan::HarnessCall::Missing,
        (true, false) => scan::HarnessCall::Unchecked,
        (true, true) => scan::HarnessCall::Checked,
    }
}

#[cfg(test)]
mod tests {
    use crate::scan;
    use crate::testing;

    #[test]
    fn test_usage_method_form_is_the_harness() {
        assert_eq!(
            testing::harness_call("{ scene.run(&c).expect(\"preview\"); }", false),
            scan::HarnessCall::Checked
        );
        assert_eq!(
            testing::harness_call("{ let _ = scene.run(&c); }", false),
            scan::HarnessCall::Unchecked
        );
        assert_eq!(
            testing::harness_call("{ scene.run(&c); }", false),
            scan::HarnessCall::Unchecked
        );
    }

    #[test]
    fn test_usage() {
        let body: syn::Block = syn::parse_quote! {{
            scene.run(&c).expect("preview");
        }};
        let call = super::from_fn_body(&body, false);
        assert_eq!(call, scan::HarnessCall::Checked);
    }

    #[test]
    fn test_usage_path_forms_and_imports() {
        assert_eq!(
            testing::harness_call(
                "{ lele_bevy_preview::run(&s, &c).expect(\"preview\"); }",
                false
            ),
            scan::HarnessCall::Checked
        );
        assert_eq!(
            testing::harness_call("{ run(&s, &c)?; }", true),
            scan::HarnessCall::Checked
        );
        assert_eq!(
            testing::harness_call("{ let _ = run(&s, &c); }", true),
            scan::HarnessCall::Unchecked
        );
        assert_eq!(
            testing::harness_call("{ run(&s, &c).expect(\"p\"); }", false),
            scan::HarnessCall::Missing
        );
        assert_eq!(
            testing::harness_call("{ server::run(); }", true),
            scan::HarnessCall::Missing
        );
    }
}
