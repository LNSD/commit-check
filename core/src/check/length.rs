//! A basic check plugin for the commit message length.

use crate::error::Error;

/// Maximum length of the commit message.
pub const MAX_MESSAGE_LEN: usize = 50;

/// Checks if the given message is valid.
pub fn check(message: &str) -> Result<(), Error> {
    if message.is_empty() {
        return Err(Error::new("empty message"));
    }

    if message.len() > MAX_MESSAGE_LEN {
        return Err(Error::new("message too long"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::{check, MAX_MESSAGE_LEN};

    #[test]
    fn invalid_empty_message() {
        //* Given
        let message = "";

        //* When
        let result = check(message);

        //* Then
        assert_matches!(result, Err(err) => {
            assert_eq!(err.to_string(), "empty message");
        });
    }

    #[test]
    fn invalid_too_long_message() {
        //* Given
        let message = "x".repeat(MAX_MESSAGE_LEN + 1);

        //* When
        let result = check(&message);

        //* Then
        assert_matches!(result, Err(err) => {
            assert_eq!(err.to_string(), "message too long");
        });
    }

    #[test]
    fn valid_message() {
        //* Given
        let message = "chore(deps): update thiserror crate to v1.0.50";

        //* When
        let result = check(message);

        //* Then
        assert_matches!(result, Ok(()));
    }
}
