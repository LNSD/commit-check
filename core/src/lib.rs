/// Maximum length of the commit message.
pub const MAX_MESSAGE_LEN: usize = 50;

/// Message validation error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty message")]
    Empty,

    #[error("message too long")]
    TooLong,
}

/// Checks if the given message is valid.
pub fn check(message: &str) -> Result<(), Error> {
    if message.is_empty() {
        return Err(Error::Empty);
    }

    if message.len() > MAX_MESSAGE_LEN {
        return Err(Error::TooLong);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::{check, Error, MAX_MESSAGE_LEN};

    #[test]
    fn invalid_empty_message() {
        //* Given
        let message = "";

        //* When
        let result = check(message);

        //* Then
        assert_matches!(result, Err(Error::Empty));
    }

    #[test]
    fn invalid_too_long_message() {
        //* Given
        let message = "x".repeat(MAX_MESSAGE_LEN + 1);

        //* When
        let result = check(&message);

        //* Then
        assert_matches!(result, Err(Error::TooLong));
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
