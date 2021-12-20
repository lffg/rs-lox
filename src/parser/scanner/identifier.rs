use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::token::TokenKind::{self, *};

/// Checks if the given char is valid as an identifier's start character.
#[inline]
pub fn is_valid_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Checks if the given char can belong to an identifier's tail.
#[inline]
pub fn is_valid_identifier_tail(c: char) -> bool {
    c.is_ascii_digit() || is_valid_identifier_start(c)
}

lazy_static! {
    pub static ref LOX_KEYWORDS: HashMap<&'static str, TokenKind> = HashMap::from([
        ("nil", Nil),
        ("true", True),
        ("false", False),
        ("this", This),
        ("super", Super),
        ("class", Class),
        ("and", And),
        ("or", Or),
        ("if", If),
        ("else", Else),
        ("return", Return),
        ("fun", Fun),
        ("for", For),
        ("while", While),
        ("var", Var),
        ("print", Print),
        ("typeof", Typeof),
        ("show", Show),
    ]);
}
