use crate::methods;
use crate::scan;

pub(crate) fn harness_call(body: &str, is_run_imported: bool) -> scan::HarnessCall {
    methods::harness_call::from_fn_body(&syn::parse_str(body).unwrap(), is_run_imported)
}

#[cfg(test)]
mod tests {
    use super::harness_call;
    use crate::scan;

    #[test]
    fn test_usage() {
        let checked = harness_call("{ scene.run(&c).expect(\"preview\"); }", false);
        assert_eq!(checked, scan::HarnessCall::Checked);
    }
}
