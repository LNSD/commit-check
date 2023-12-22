use assert_matches::assert_matches;

use commit_check_core::check;

#[test]
fn valid_message() {
    //* Given
    let message = "chore(deps): update thiserror crate to 1.0.50";

    //* When
    let result = check(message);

    //* Then
    assert_matches!(result, Ok(()));
}
