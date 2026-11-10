use crate::basic::constants;

pub struct SnakeCaseWalk {
    chars: Vec<char>,
    result: String,
    skip_until: usize,
}

impl From<&str> for SnakeCaseWalk {
    fn from(pascal: &str) -> Self {
        Self {
            chars: pascal.chars().collect(),
            result: String::new(),
            skip_until: 0,
        }
    }
}

#[rustfmt::skip]
impl SnakeCaseWalk {
    pub(crate) fn snake_cased(mut self) -> String {
        let chars = self.chars.clone();
        for (position, c) in chars.iter().copied().enumerate() {
            self.push_char(position, c);
        }
        self.result
    }

    pub(crate) fn push_char(&mut self, position: usize, c: char) {
        if position >= self.skip_until {
            self.push_word_start(position, c);
        }
    }

    pub(crate) fn push_word_start(&mut self, position: usize, c: char) {
        if let Some(word) = self.acronym_at(position) {
            self.push_acronym(word, position);
        } else {
            self.push_scalar_char(position, c);
        }
    }

    pub(crate) fn push_acronym(&mut self, word: &'static str, position: usize) {
        self.push_word_underscore();
        self.result.push_str(&word.to_ascii_lowercase());
        self.skip_until = position.saturating_add(word.chars().count());
    }

    pub(crate) fn push_word_underscore(&mut self) {
        if !self.result.is_empty() && !self.result.ends_with('_') {
            self.result.push('_');
        }
    }

    pub(crate) fn push_scalar_char(&mut self, position: usize, c: char) {
        if c.is_uppercase() {
            self.push_uppercase_char(position, c);
        } else {
            self.result.push(c);
        }
    }

    pub(crate) fn push_uppercase_char(&mut self, position: usize, c: char) {
        if self.starts_new_word(position) {
            self.result.push('_');
        }
        self.result.push(c.to_ascii_lowercase());
    }

    pub(crate) fn starts_new_word(&self, position: usize) -> bool {
        self.preceded_by_lowercase(position)
            || (self.followed_by_lowercase(position) && self.preceded_by_uppercase(position))
    }

    pub(crate) fn preceded_by_lowercase(&self, position: usize) -> bool {
        position > 0
            && self
                .chars
                .get(position.saturating_sub(1))
                .is_some_and(|p| p.is_lowercase())
    }

    pub(crate) fn preceded_by_uppercase(&self, position: usize) -> bool {
        position > 0
            && self
                .chars
                .get(position.saturating_sub(1))
                .is_some_and(|p| p.is_uppercase())
    }

    pub(crate) fn followed_by_lowercase(&self, position: usize) -> bool {
        self.chars
            .get(position.saturating_add(1))
            .is_some_and(|p| p.is_lowercase())
    }

    pub(crate) fn acronym_at(&self, position: usize) -> Option<&'static str> {
        if self.is_word_start(position) {
            constants::ACRONYMS
                .iter()
                .copied()
                .find(|&acronym| self.matches_acronym(position, acronym))
        } else {
            None
        }
    }

    pub(crate) fn is_word_start(&self, position: usize) -> bool {
        position == 0
            || self
                .chars
                .get(position.saturating_sub(1))
                .is_some_and(|p| !p.is_uppercase())
    }

    pub(crate) fn matches_acronym(&self, start: usize, acronym: &'static str) -> bool {
        let matched_chars = acronym
            .chars()
            .enumerate()
            .all(|(offset, expected)| {
                self.chars.get(start.saturating_add(offset)) == Some(&expected)
            });
        let ends_on_word_boundary = self.ends_on_word_boundary(start, acronym);
        matched_chars && ends_on_word_boundary
    }

    pub(crate) fn ends_on_word_boundary(&self, start: usize, acronym: &'static str) -> bool {
        let end = start.saturating_add(acronym.chars().count());
        self.chars.get(end).is_none_or(|c| !c.is_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::SnakeCaseWalk;

    #[test]
    fn test_usage() {
        let acronyms = SnakeCaseWalk::from("P2PPlugin");
        assert_eq!(acronyms.snake_cased(), "p2p_plugin");
        assert_eq!(
            SnakeCaseWalk::from("P2PRoomDiscoveryPlugin").snake_cased(),
            "p2p_room_discovery_plugin"
        );
        assert_eq!(
            SnakeCaseWalk::from("MyP2PThing").snake_cased(),
            "my_p2p_thing"
        );
        assert_eq!(
            SnakeCaseWalk::from("ConstructorNoSkip").snake_cased(),
            "constructor_no_skip"
        );
        assert_eq!(SnakeCaseWalk::from("Player").snake_cased(), "player");
        assert_eq!(
            SnakeCaseWalk::from("PlayerEvent").snake_cased(),
            "player_event"
        );
    }
}
