/// Returns an "humanized" string representation of the given character.
#[inline]
pub fn humanized_char(c: char) -> String {
    match c {
        '\0' => "eof".into(),
        c => c.into(),
    }
}

/// Checks if the given char is a valid identifier start.
#[inline]
pub fn is_valid_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Checks if the given char is a valid identifier end.
#[inline]
pub fn is_valid_identifier_end(c: char) -> bool {
    c.is_ascii_digit() || is_valid_identifier_start(c)
}
