use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum LoxValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

// Utilities.
impl LoxValue {
    pub fn type_name(&self) -> &'static str {
        use LoxValue::*;
        match self {
            Boolean(_) => "boolean",
            Number(_) => "number",
            String(_) => "string",
            Nil => "nil",
        }
    }
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxValue::*;
        match self {
            Boolean(b) => b.fmt(f),
            Number(n) => {
                if n.floor() == *n {
                    write!(f, "{:.0}", n)
                } else {
                    n.fmt(f)
                }
            }
            String(s) => write!(f, "\"{}\"", s),
            Nil => f.write_str("nil"),
        }
    }
}
