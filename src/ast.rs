macro_rules! make_enum {
    ( $enum_name:ident, [ $( $variant:ident ),* $( , )? ] ) => {
        #[derive(Debug)]
        pub enum $enum_name {
            $( $variant($variant), )*
        }
        $(
            impl From<$variant> for $enum_name {
                fn from(val: $variant) -> $enum_name {
                    $enum_name::$variant(val)
                }
            }
            #[allow(clippy::from_over_into)]
            impl Into<Box<$enum_name>> for $variant {
                fn into(self) -> Box<$enum_name> {
                    Box::new($enum_name::from(self))
                }
            }
        )*
    }
}

pub mod expr {
    use crate::{
        span::Span,
        token::{Token, TokenKind},
    };

    #[derive(Debug)]
    pub struct Expr {
        pub kind: ExprKind,
        pub span: Span,
    }

    make_enum!(ExprKind, [Literal, Group, Unary, Binary]);

    impl ExprKind {
        /// Converts the `ExprKind` into an `Expr` given a span.
        pub fn into_expr(self, span: Span) -> Expr {
            Expr { kind: self, span }
        }
    }

    #[derive(Debug)]
    pub enum Literal {
        Boolean(bool),
        Number(f64),
        String(String),
        Nil,
    }

    impl From<Token> for Literal {
        fn from(token: Token) -> Self {
            use TokenKind as T;
            match token.kind {
                T::String(inner) => Literal::String(inner),
                T::Number(inner) => Literal::Number(inner),
                T::False => Literal::Boolean(false),
                T::True => Literal::Boolean(true),
                T::Nil => Literal::Nil,
                _ => panic!("Invalid `Token` to `Literal` conversion. This is a bug."),
            }
        }
    }

    #[derive(Debug)]
    pub struct Group {
        pub expr: Box<Expr>,
    }

    #[derive(Debug)]
    pub struct Unary {
        pub operator: Token,
        pub operand: Box<Expr>,
    }

    #[derive(Debug)]
    pub struct Binary {
        pub left: Box<Expr>,
        pub operator: Token,
        pub right: Box<Expr>,
    }
}
