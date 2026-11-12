use crate::Identifier;

pub fn to_pascal_case(identifier: &Identifier) -> String {
    identifier
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_ascii_uppercase().to_string() + &chars.as_str().to_lowercase()
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::to_pascal_case;
    use crate::Identifier;

    #[test]
    fn test_usage() {
        assert_eq!(to_pascal_case(&Identifier("click_counter")), "ClickCounter");
        assert_eq!(to_pascal_case(&Identifier("ui_ROOT")), "UiRoot");
    }
}
