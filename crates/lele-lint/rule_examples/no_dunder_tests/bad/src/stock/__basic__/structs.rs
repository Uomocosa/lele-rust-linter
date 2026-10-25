pub struct Item {
    pub name: String,
    pub price: u32,
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_usage() {
        let item = Item {
            name: "apple".to_string(),
            price: 3,
        };
        assert_eq!(item.price, 3);
    }
}
