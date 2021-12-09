use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::token::TokenKind::{self, *};

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
    ]);
}
