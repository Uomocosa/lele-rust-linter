use crate::FnBody;

pub fn is_delegate_call(block: FnBody) -> bool {
    let Some(segments) = block.single_path_call_segments() else {
        return false;
    };
    let [.., module, _] = segments.as_slice() else {
        return false;
    };
    let Some(first) = segments.first() else {
        return false;
    };
    if matches!(
        first.as_str(),
        "Self" | "self" | "crate" | "std" | "core" | "alloc"
    ) {
        return false;
    }
    !(module.starts_with(char::is_uppercase)
        || matches!(
            module.as_str(),
            "u8" | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "usize"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "isize"
                | "f32"
                | "f64"
                | "bool"
                | "char"
                | "str"
        ))
}

#[cfg(test)]
mod tests {
    use super::is_delegate_call;
    use crate::FnBody;
    use syn::Block;

    #[test]
    fn test_usage() {
        let delegate: Block = syn::parse_str("{ player_heal::heal(self, amount) }").unwrap();
        assert!(is_delegate_call(FnBody(&delegate)));

        let methods_path: Block = syn::parse_str("{ methods::player::heal(self) }").unwrap();
        assert!(is_delegate_call(FnBody(&methods_path)));

        let constructor: Block = syn::parse_str("{ config_new::new() }").unwrap();
        assert!(is_delegate_call(FnBody(&constructor)));

        let no_arguments: Block = syn::parse_str("{ helpers::now() }").unwrap();
        assert!(is_delegate_call(FnBody(&no_arguments)));

        let std_call: Block = syn::parse_str("{ std::mem::take(&mut self.unanswered) }").unwrap();
        assert!(!is_delegate_call(FnBody(&std_call)));

        let core_call: Block = syn::parse_str("{ core::mem::swap(a, b) }").unwrap();
        assert!(!is_delegate_call(FnBody(&core_call)));

        let primitive: Block = syn::parse_str("{ u64::from(self.count) }").unwrap();
        assert!(!is_delegate_call(FnBody(&primitive)));

        let wrapper: Block = syn::parse_str("{ Wrapper::from(self) }").unwrap();
        assert!(!is_delegate_call(FnBody(&wrapper)));

        let self_call: Block = syn::parse_str("{ Self::base(self.hits) }").unwrap();
        assert!(!is_delegate_call(FnBody(&self_call)));

        let struct_lit: Block = syn::parse_str("{ Self { x: 1 } }").unwrap();
        assert!(!is_delegate_call(FnBody(&struct_lit)));

        let with_let: Block = syn::parse_str("{ let a = 1; Foo { x: a } }").unwrap();
        assert!(!is_delegate_call(FnBody(&with_let)));
    }
}
