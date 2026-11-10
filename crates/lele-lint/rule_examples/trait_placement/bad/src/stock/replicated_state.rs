pub trait ReplicatedState {
    fn replica_id(&self) -> u64;
}
