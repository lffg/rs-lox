use lox::{
    parser::scanner::Scanner,
    token::TokenKind::{self, *},
};

mod helpers;
use helpers::multi_test::MultiTest;

lazy_static::lazy_static! {
    static ref COMMON_TOKENS: Vec<(&'static str, &'static str, TokenKind)> = vec![
        //
        // Common tokens
        //
        ("left_paren_token", "(", LeftParen),
        ("right_paren_token", ")", RightParen),
        ("left_brace_token", "{", LeftBrace),
        ("right_brace_token", "}", RightBrace),
        ("plus_token", "+", Plus),
        ("minus_token", "-", Minus),
        ("star_token", "*", Star),
        ("slash_token", "/", Slash),
        ("dot_token", ".", Dot),
        ("comma_token", ",", Comma),
        ("semicolon_token", ";", Semicolon),
        ("bang_token", "!", Bang),
        ("bang_equal_token", "!=", BangEqual),
        ("equal_token", "=", Equal),
        ("equal_equal_token", "==", EqualEqual),
        ("less_token", "<", Less),
        ("less_equal_token", "<=", LessEqual),
        ("greater_token", ">", Greater),
        ("greater_equal_token", ">=", GreaterEqual),
        ("nil_token", "nil", Nil),
        ("true_token", "true", True),
        ("false_token", "false", False),
        ("this_token", "this", This),
        ("super_token", "super", Super),
        ("class_token", "class", Class),
        ("and_token", "and", And),
        ("or_token", "or", Or),
        ("if_token", "if", If),
        ("else_token", "else", Else),
        ("return_token", "return", Return),
        ("fun_token", "fun", Fun),
        ("for_token", "for", For),
        ("while_token", "while", While),
        ("var_token", "var", Var),
        ("print_token", "print", Print),
        ("typeof_token", "typeof", Typeof),
        ("show_token", "show", Show),
        //
        // Identifier tokens
        //
        ("identifier_token_1", "r2d2", Identifier("r2d2".into())),
        ("identifier_token_2", "r2_d2", Identifier("r2_d2".into())),
        ("identifier_token_3", "r2_2d", Identifier("r2_2d".into())),
        //
        // Number tokens
        //
        ("number_token_1", "3", Number(3.0)),
        ("number_token_2", "3.14", Number(3.14)),
        //
        // String tokens
        //
        ("string_token_1", r#""""#, String("".into())),
        ("string_token_2", r#"" a ""#, String(" a ".into())),
        ("string_token_3", r#""aaa""#, String("aaa".into())),
    ];

    static ref OTHER_TOKENS: Vec<(&'static str, &'static str, TokenKind)> = vec![
        //
        // Comment tokens
        //
        ("comment_token_1", "//", Comment("".into())),
        ("comment_token_2", "///", Comment("/".into())),
        ("comment_token_3", "// foo", Comment(" foo".into())),
        //
        // Whitespace tokens
        //
        ("whitespace_token_1", " ", Whitespace(" ".into())),
        ("whitespace_token_2", "  ", Whitespace("  ".into())),
        ("whitespace_token_3", "\t", Whitespace("\t".into())),
        ("whitespace_token_4", "\n", Whitespace("\n".into())),
        ("whitespace_token_5", "\r", Whitespace("\r".into())),
        ("whitespace_token_6", "\r\n", Whitespace("\r\n".into())),
        //
        // Some errors
        //
        ("string_error_1", r#"""#, Error("Unterminated string".into())),
        ("string_error_2", r#""a"#, Error("Unterminated string".into())),
        ("unexpected_token_1", "%", Error("Unexpected character `%`".into())),
        ("unexpected_token_2", "&", Error("Unexpected character `&`".into())),
    ];
}

#[test]
fn single_token() {
    let mut mt: MultiTest = MultiTest::new();
    let tokens = COMMON_TOKENS.iter().chain(OTHER_TOKENS.iter());
    for (case_name, src_str, expected_kind) in tokens {
        mt.named_test(case_name, move || {
            let mut scanner = Scanner::new(src_str);
            assert_eq!(scanner.next().unwrap().kind, *expected_kind);
            assert_eq!(scanner.next().unwrap().kind, Eof);
        });
    }
}

#[test]
pub fn token_pairs() {
    let mut mt: MultiTest<std::string::String> = MultiTest::new();
    for (name1, src1, kind1) in COMMON_TOKENS.iter() {
        for (name2, src2, kind2) in COMMON_TOKENS.iter() {
            if is_separation_required(kind1, kind2) {
                continue;
            }
            let src = format!("{}{}", src1, src2);
            let name = format!("pair with {} and {} ({})", name1, name2, &src);
            mt.named_test(name, move || {
                let mut scanner = Scanner::new(&src);
                assert_eq!(scanner.next().unwrap().kind, *kind1);
                assert_eq!(scanner.next().unwrap().kind, *kind2);
                assert_eq!(scanner.next().unwrap().kind, Eof);
            });
        }
    }
}

#[test]
pub fn token_pairs_with_separator() {
    let mut mt: MultiTest<std::string::String> = MultiTest::new();
    for (name1, src1, kind1) in COMMON_TOKENS.iter() {
        for (name2, src2, kind2) in COMMON_TOKENS.iter() {
            let src = format!("{} {}", src1, src2);
            let name = format!("separated pair with {} and {} ({})", name1, name2, &src);
            mt.named_test(name, move || {
                let mut scanner = Scanner::new(&src);
                assert_eq!(scanner.next().unwrap().kind, *kind1);
                assert_eq!(scanner.next().unwrap().kind, Whitespace(" ".into()));
                assert_eq!(scanner.next().unwrap().kind, *kind2);
                assert_eq!(scanner.next().unwrap().kind, Eof);
            });
        }
    }
}

fn is_separation_required(kind1: &TokenKind, kind2: &TokenKind) -> bool {
    match (kind1, kind2) {
        (Slash, Slash) => true,
        (Equal, Equal) => true,
        (Equal, EqualEqual) => true,
        (Bang, Equal) => true,
        (Bang, EqualEqual) => true,
        (Less, Equal) => true,
        (Less, EqualEqual) => true,
        (Greater, Equal) => true,
        (Greater, EqualEqual) => true,

        (Number(_), Number(_)) => true,
        (a, Number(_)) if a.is_keyword() => true,

        (Identifier(_), Identifier(_)) => true,
        (Identifier(_), Number(_)) => true,
        (a, Identifier(_)) if a.is_keyword() => true,
        (Identifier(_), b) if b.is_keyword() => true,

        (a, b) if a.is_keyword() && b.is_keyword() => true,
        _ => false,
    }
}
