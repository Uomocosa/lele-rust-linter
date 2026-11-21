pub fn sensor_read(stamp: u64, buffer: &mut Vec<u64>) {
    buffer.push(stamp);
    tracing::debug!(target: "sensor", stamp = stamp);
}

#[cfg(test)]
mod tests {
    use super::sensor_read;

    #[test]
    fn test_usage() {
        let mut buffer = Vec::new();
        sensor_read(3, &mut buffer);
        assert_eq!(buffer, vec![3]);
    }
}
