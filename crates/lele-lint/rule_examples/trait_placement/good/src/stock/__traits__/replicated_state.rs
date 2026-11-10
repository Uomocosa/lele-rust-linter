pub trait ReplicatedState {
    fn replica_id(&self) -> u64;
}

#[cfg(test)]
mod tests {
    use super::ReplicatedState;

    struct Demo {
        replica: u64,
        owner: u32,
    }

    impl ReplicatedState for Demo {
        fn replica_id(&self) -> u64 {
            self.replica + u64::from(self.owner)
        }
    }

    #[test]
    fn test_usage() {
        let demo = Demo {
            replica: 6,
            owner: 1,
        };
        assert_eq!(demo.replica_id(), 7);
    }
}
