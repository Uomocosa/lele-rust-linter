use crate::clicker;

pub fn increment(counter: &mut clicker::ClickCounter) {
    if **counter == i32::MAX {
        **counter = i32::MIN;
    } else {
        **counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::increment;
    use crate::clicker;

    #[test]
    fn test_usage() {
        let mut counter = clicker::ClickCounter::default();
        increment(&mut counter);
        assert_eq!(*counter, 1);
    }
}
