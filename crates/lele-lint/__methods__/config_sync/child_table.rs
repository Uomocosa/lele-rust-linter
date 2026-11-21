pub fn child_table<'table>(
    table: &'table mut toml_edit::Table,
    key: &str,
) -> Option<&'table mut toml_edit::Table> {
    if !table.contains_key(key) {
        let mut created = toml_edit::Table::new();
        created.set_implicit(true);
        table.insert(key, toml_edit::Item::Table(created));
    }
    table.get_mut(key)?.as_table_mut()
}

#[cfg(test)]
mod tests {
    use super::child_table;

    #[test]
    fn test_usage() {
        let mut table = toml_edit::Table::new();
        let child = child_table(&mut table, "lele").unwrap();
        assert!(child.is_empty());
        assert!(table.contains_key("lele"));
    }

    #[test]
    fn non_table_value_is_left_alone() {
        let mut table = toml_edit::Table::new();
        table["lele"] = toml_edit::value(5);
        assert!(child_table(&mut table, "lele").is_none());
    }
}
