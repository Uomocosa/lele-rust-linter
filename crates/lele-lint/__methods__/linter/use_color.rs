use std::io::IsTerminal as _;

pub fn use_color() -> bool {
    std::env::var_os("CLICOLOR_FORCE").is_some_and(|value| value != "0")
        || (std::env::var_os("NO_COLOR").is_none() && std::io::stderr().is_terminal())
}

#[cfg(test)]
mod tests {
    use super::use_color;

    #[test]
    fn test_usage() {
        assert_eq!(use_color(), use_color());
    }
}
