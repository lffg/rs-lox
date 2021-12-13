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
            impl Into<Box<$enum_name>> for $variant {
                fn into(self) -> Box<$enum_name> {
                    Box::new($enum_name::from(self))
                }
            }
        )*
    }
}

pub mod expr {
    use crate::token::TokenKind;

    make_enum!(Expr, [Literal, Group, Unary, Binary]);

    // =============================================================================================
    // Literal

    #[derive(Debug)]
    pub enum Literal {
        Boolean(bool),
        Number(f64),
        String(String),
        Nil,
    }

    impl From<TokenKind> for Literal {
        fn from(kind: TokenKind) -> Self {
            use TokenKind as T;
            match kind {
                T::String(inner) => Literal::String(inner),
                T::Number(inner) => Literal::Number(inner),
                T::False => Literal::Boolean(false),
                T::True => Literal::Boolean(true),
                T::Nil => Literal::Nil,
                _ => panic!("Invalid `TokenKind` to `Literal` conversion. This is a bug."),
            }
        }
    }

    // =============================================================================================
    // Grouping

    #[derive(Debug)]
    pub struct Group {
        pub inner: Box<Expr>,
    }

    // =============================================================================================
    // Unary

    #[derive(Debug)]
    pub enum UnaryOperator {
        Minus,
        Bang,
    }

    impl From<TokenKind> for UnaryOperator {
        fn from(kind: TokenKind) -> Self {
            use TokenKind as T;
            use UnaryOperator as U;
            match kind {
                T::Minus => U::Minus,
                T::Bang => U::Bang,
                _ => panic!("Invalid `TokenKind` to `UnaryOperator` conversion. This is a bug."),
            }
        }
    }

    #[derive(Debug)]
    pub struct Unary {
        pub operator: UnaryOperator,
        pub operand: Box<Expr>,
    }

    // =============================================================================================
    // Binary

    #[derive(Debug)]
    pub enum BinaryOperator {
        Plus,
        Minus,
        Star,
        Slash,
        EqualEqual,
        BangEqual,
        Less,
        LessEqual,
        Greater,
        GreaterEqual,
    }

    impl From<TokenKind> for BinaryOperator {
        fn from(kind: TokenKind) -> Self {
            use BinaryOperator as B;
            use TokenKind as T;
            match kind {
                T::Plus => B::Plus,
                T::Minus => B::Minus,
                T::Star => B::Star,
                T::Slash => B::Slash,
                T::EqualEqual => B::EqualEqual,
                T::BangEqual => B::BangEqual,
                T::Less => B::Less,
                T::LessEqual => B::LessEqual,
                T::Greater => B::Greater,
                T::GreaterEqual => B::GreaterEqual,
                _ => panic!("Invalid `TokenKind` to `BinaryOperator` conversion. This is a bug."),
            }
        }
    }

    #[derive(Debug)]
    pub struct Binary {
        pub left: Box<Expr>,
        pub operator: BinaryOperator,
        pub right: Box<Expr>,
    }
}
