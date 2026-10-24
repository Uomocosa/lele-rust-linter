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
    use super::from_fn_body;
    use crate::scan::HarnessCall;

    fn call(body: &str, is_run_imported: bool) -> HarnessCall {
        from_fn_body(&syn::parse_str(body).unwrap(), is_run_imported)
    }

    #[test]
    fn test_usage_method_form_is_the_harness() {
        assert_eq!(
            call("{ scene.run(&c, \"x\").expect(\"preview\"); }", false),
            HarnessCall::Checked
        );
        assert_eq!(
            call("{ let _ = scene.run(&c, \"x\"); }", false),
            HarnessCall::Unchecked
        );
        assert_eq!(
            call("{ scene.run(&c, \"x\"); }", false),
            HarnessCall::Unchecked
        );
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            call(
                "{ lele_bevy_preview::run(&s, &c, \"x\").expect(\"preview\"); }",
                false
            ),
            HarnessCall::Checked
        );
        assert_eq!(call("{ run(&s, &c, \"x\")?; }", true), HarnessCall::Checked);
        assert_eq!(
            call("{ let _ = run(&s, &c, \"x\"); }", true),
            HarnessCall::Unchecked
        );
        assert_eq!(
            call("{ run(&s, &c, \"x\").expect(\"p\"); }", false),
            HarnessCall::Missing
        );
        assert_eq!(call("{ server::run(); }", true), HarnessCall::Missing);
    }
}
