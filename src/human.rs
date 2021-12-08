/// Returns an "humanized" string representation of the given character.
#[inline]
pub fn char(c: char) -> String {
    match c {
        '\0' => "eof".into(),
        c => c.into(),
    }
}
