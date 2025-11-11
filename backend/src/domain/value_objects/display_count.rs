/// DisplayCount value object
#[derive(Debug, Clone, Copy)]
pub struct DisplayCount(i32);

impl DisplayCount {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_value(value: i32) -> Self {
        Self(value)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn is_expired(&self) -> bool {
        self.0 >= 100
    }
}

impl Default for DisplayCount {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_new_display_count() {
        let count = DisplayCount::new();
        assert_eq!(count.value(), 0);
    }

    #[rstest]
    fn test_default_display_count() {
        let count = DisplayCount::default();
        assert_eq!(count.value(), 0);
    }

    #[rstest]
    #[case(0)]
    #[case(5)]
    #[case(10)]
    #[case(100)]
    fn test_from_value(#[case] value: i32) {
        let count = DisplayCount::from_value(value);
        assert_eq!(count.value(), value);
    }

    #[rstest]
    fn test_increment() {
        let mut count = DisplayCount::new();
        count.increment();
        assert_eq!(count.value(), 1);
        count.increment();
        assert_eq!(count.value(), 2);
    }

    #[rstest]
    #[case(0, false)]
    #[case(5, false)]
    #[case(9, false)]
    #[case(10, false)]
    #[case(50, false)]
    #[case(99, false)]
    #[case(100, true)]
    #[case(101, true)]
    fn test_is_expired(#[case] count_value: i32, #[case] expected: bool) {
        let count = DisplayCount::from_value(count_value);
        assert_eq!(count.is_expired(), expected);
    }

    #[rstest]
    fn test_increment_until_expired() {
        let mut count = DisplayCount::new();
        assert!(!count.is_expired());

        for _ in 0..100 {
            count.increment();
        }
        assert!(count.is_expired());
    }
}
