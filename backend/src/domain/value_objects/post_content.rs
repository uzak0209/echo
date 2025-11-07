use crate::domain::error::ValidationError;

/// PostContent value object
#[derive(Debug, Clone)]
pub struct PostContent(String);

impl PostContent {
    pub fn new(content: String) -> Result<Self, ValidationError> {
        if content.is_empty() {
            return Err(ValidationError::EmptyContent);
        }
        if content.len() > 1000 {
            return Err(ValidationError::ContentTooLong {
                max: 1000,
                actual: content.len(),
            });
        }
        Ok(Self(content))
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
    #[case("Valid content")]
    #[case("a")]
    #[case("Short")]
    fn test_new_valid_content(#[case] input: &str) {
        let content = PostContent::new(input.to_string());
        assert!(content.is_ok());
        assert_eq!(content.unwrap().value(), input);
    }

    #[rstest]
    fn test_new_valid_max_length_content() {
        let input = "A".repeat(1000);
        let content = PostContent::new(input.clone());
        assert!(content.is_ok());
        assert_eq!(content.unwrap().value(), input);
    }

    #[rstest]
    fn test_new_empty_content() {
        let content = PostContent::new("".to_string());
        assert!(matches!(content, Err(ValidationError::EmptyContent)));
    }

    #[rstest]
    #[case(1001)]
    #[case(1500)]
    #[case(2000)]
    fn test_new_too_long_content(#[case] length: usize) {
        let long_content = "a".repeat(length);
        let content = PostContent::new(long_content);
        assert!(matches!(
            content,
            Err(ValidationError::ContentTooLong { .. })
        ));
    }

    #[rstest]
    fn test_value_returns_correct_string() {
        let input = "Test content";
        let content = PostContent::new(input.to_string()).unwrap();
        assert_eq!(content.value(), input);
    }
}
