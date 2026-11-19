pub trait ReplicatedState {
    fn replica_id(&self) -> u64;
}

#[rustfmt::skip]
impl ReplicatedState for u64 {
    fn replica_id(&self) -> u64 { *self }
}

#[cfg(test)]
mod tests {
    use super::ReplicatedState;

    #[test]
    fn test_usage() {
        assert_eq!(7_u64.replica_id(), 7);
    }
}
