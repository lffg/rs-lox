macro_rules! make_ast_enum {
    ( $enum_name:ident, [ $( $variant:ident ),* $( , )? ] ) => {
        #[derive(Debug, Clone)]
        pub enum $enum_name {
            $( $variant($variant), )*
        }
        impl $enum_name {
            /// Returns the span of the inner AST node.
            #[inline]
            pub fn span(&self) -> Span {
                match self {
                    $(
                        $enum_name::$variant(inner) => inner.span,
                    )*
                }
            }
        }
        $(
            impl From<$variant> for $enum_name {
                fn from(val: $variant) -> $enum_name {
                    $enum_name::$variant(val)
                }
            }
            impl From<$variant> for Box<$enum_name> {
                fn from(variant: $variant) -> Box<$enum_name> {
                    Box::new($enum_name::from(variant))
                }
            }
        )*
    }
}

pub mod dbg;
pub mod expr;
pub mod stmt;
