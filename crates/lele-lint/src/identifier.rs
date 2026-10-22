use std::collections::HashSet;

use derive_more::Deref;
use lele_snake_case::ToLeleSnakeCase;

#[derive(Clone, Copy, Deref)]
pub struct Identifier<'a>(pub &'a str);

#[rustfmt::skip]
impl<'a> Identifier<'a> {
    pub fn to_pascal_case(&self) -> String {
        self.split('_')
            .map(|word| {
                let mut chars = word.chars();
                chars.next().map_or_else(String::new, |first| first.to_ascii_uppercase().to_string() + &chars.as_str().to_lowercase())
            })
            .collect()
    }
    pub fn is_stuttered_in(&self, module: &str) -> bool {
        self.chars().next().is_some_and(|c| c.is_uppercase()) && self.to_snake_case() == module
    }
    pub fn longest_type_prefix(&self, known: &HashSet<String>) -> Option<&'a str> {
        let stem: &'a str = **self;
        stem.match_indices('_').rev().find_map(|(pos, _)| stem.get(..pos).filter(|prefix| known.contains(*prefix)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::Identifier;

    #[test]
    fn test_usage() {
        assert_eq!(Identifier("click_counter").to_pascal_case(), "ClickCounter");
        assert!(Identifier("Meters").is_stuttered_in("meters"));
        assert!(!Identifier("walk_entries").is_stuttered_in("walk_entries"));
        let known: HashSet<String> = std::iter::once("freenet_client".to_string()).collect();
        assert_eq!(
            Identifier("freenet_client_connect").longest_type_prefix(&known),
            Some("freenet_client")
        );
        assert_eq!(Identifier("other_thing").longest_type_prefix(&known), None);
    }
}
