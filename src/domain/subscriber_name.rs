use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_character = s.chars().any(|g| forbidden.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_character {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}
