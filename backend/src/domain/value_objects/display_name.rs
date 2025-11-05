/// DisplayName value object
#[derive(Debug, Clone)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("TestUser")]
    #[case("Anonymous")]
    #[case("User123")]
    #[case("ユーザー")]
    fn test_new_display_name(#[case] input: &str) {
        let name = DisplayName::new(input.to_string());
        assert_eq!(name.value(), input);
    }

    #[rstest]
    fn test_value_returns_correct_string() {
        let input = "TestUser";
        let name = DisplayName::new(input.to_string());
        assert_eq!(name.value(), input);
    }
}
