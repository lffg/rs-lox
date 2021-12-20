use std::fmt::{self, Debug, Display};

#[derive(Clone)]
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
            Boolean(b) => Display::fmt(b, f),
            Number(n) => {
                if n.floor() == *n {
                    write!(f, "{:.0}", n)
                } else {
                    Display::fmt(n, f)
                }
            }
            String(s) => f.write_str(s),
            Nil => f.write_str("nil"),
        }
    }
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxValue::*;
        match self {
            String(s) => write!(f, "\"{}\"", s),
            other => Display::fmt(other, f),
        }
    }
}
