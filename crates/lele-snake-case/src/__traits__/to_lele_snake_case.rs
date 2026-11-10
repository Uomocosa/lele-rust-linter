use crate::SnakeCaseWalk;

pub trait ToLeleSnakeCase {
    #[must_use]
    fn to_snake_case(&self) -> String;
}

#[rustfmt::skip]
impl ToLeleSnakeCase for str {
    fn to_snake_case(&self) -> String { SnakeCaseWalk::from(self).snake_cased() }
}

#[cfg(test)]
mod tests {
    use super::ToLeleSnakeCase;

    #[test]
    fn test_usage() {
        assert_eq!("ConstructorNoSkip".to_snake_case(), "constructor_no_skip");
        assert_eq!("DomainImport".to_snake_case(), "domain_import");
        assert_eq!("SnakeCaseFiles".to_snake_case(), "snake_case_files");
        assert_eq!("NoPositional".to_snake_case(), "no_positional");
        assert_eq!("Player".to_snake_case(), "player");
        assert_eq!("PlayerEvent".to_snake_case(), "player_event");
    }

    #[test]
    fn test_usage_acronym_boundaries() {
        assert_eq!("P2PPlugin".to_snake_case(), "p2p_plugin");
        assert_eq!(
            "P2PRoomDiscoveryPlugin".to_snake_case(),
            "p2p_room_discovery_plugin"
        );
        assert_eq!("P2PEvents".to_snake_case(), "p2p_events");
        assert_eq!("P2P".to_snake_case(), "p2p");
        assert_eq!("MyP2PThing".to_snake_case(), "my_p2p_thing");
        assert_eq!("NetworkId".to_snake_case(), "network_id");
        assert_eq!("HTTPSConnection".to_snake_case(), "https_connection");
        assert_eq!("IOStream".to_snake_case(), "io_stream");
    }

    #[test]
    fn test_usage_lowercase_acronym_falls_back() {
        assert_eq!("P2pPlugin".to_snake_case(), "p2p_plugin");
        assert_eq!("P2Plus".to_snake_case(), "p2plus");
    }
}
