/// Dial timeout in milliseconds.
/// Copied from the libp2p Kademlia default and measured at 200ms p99 in CI.
pub const DIAL_TIMEOUT_MS: u64 = 10_000;
