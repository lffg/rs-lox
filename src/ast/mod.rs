use std::sync::atomic::{self, AtomicUsize};

macro_rules! make_ast_enum {
    ( $enum_name:ident, [ $( $variant:ident ),* $( , )? ] ) => {
        #[derive(Debug, Clone)]
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

pub mod dbg;
pub mod expr;
pub mod stmt;

// Yep, global state:
static AST_ID_SEQ: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AstId(usize);

impl AstId {
    pub fn new() -> Self {
        AstId(AST_ID_SEQ.fetch_add(1, atomic::Ordering::SeqCst))
    }
}
