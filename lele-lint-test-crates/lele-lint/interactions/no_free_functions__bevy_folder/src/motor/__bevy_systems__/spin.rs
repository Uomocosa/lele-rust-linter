pub fn spin(revolutions: u64, buffer: &mut Vec<u64>) {
    buffer.push(revolutions);
    tracing::debug!(target: "motor", revolutions = revolutions);
}

#[cfg(test)]
mod tests {
    use super::spin;

    #[test]
    fn test_usage() {
        let mut buffer = Vec::new();
        spin(9, &mut buffer);
        assert_eq!(buffer, vec![9]);
    }
}
