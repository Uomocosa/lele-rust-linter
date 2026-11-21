pub fn widget_poll(stamp: u64, buffer: &mut Vec<u64>) {
    buffer.push(stamp);
    tracing::debug!(target: "widget", stamp = stamp);
}

#[cfg(test)]
mod tests {
    use super::widget_poll;

    #[test]
    fn test_usage() {
        let mut buffer = Vec::new();
        widget_poll(7, &mut buffer);
        assert_eq!(buffer, vec![7]);
    }
}
