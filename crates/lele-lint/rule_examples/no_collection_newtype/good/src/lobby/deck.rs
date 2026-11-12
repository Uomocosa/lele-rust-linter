use crate::lobby;
use derive_more::Deref;

#[derive(Deref)]
pub struct Deck(Vec<lobby::Card>);

impl TryFrom<Vec<lobby::Card>> for Deck {
    type Error = Vec<lobby::Card>;
    fn try_from(cards: Vec<lobby::Card>) -> Result<Self, Self::Error> {
        if cards.len() > 52 { Err(cards) } else { Ok(Self(cards)) }
    }
}

#[cfg(test)]
mod tests {
    use super::Deck;

    #[test]
    fn test_usage() {
        assert!(Deck::try_from(Vec::new()).is_ok());
    }
}
