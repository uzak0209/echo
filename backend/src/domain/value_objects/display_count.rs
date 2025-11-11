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

    pub fn value(&self) -> i32 {
        self.0
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
}
