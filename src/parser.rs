use std::iter::Peekable;

use anyhow::{bail, Result};

use crate::{
    ast::expr::{Binary, Expr, ExprKind, Group, Literal, Unary},
    scanner::Scanner,
    token::{Token, TokenKind},
};

pub struct Parser<'src> {
    scanner: Peekable<Scanner<'src>>,
    current_token: Token,
}

macro_rules! binary_expression {
    ($self:expr, kinds = $( $kind:ident )|+, next_production = $next:ident) => {{
        let mut expr = $self.$next()?;
        while let $( TokenKind::$kind )|+ = $self.current_token.kind {
            let operator = $self.advance().clone();
            let right = $self.$next()?;
            let span = expr.span.to(right.span);
            let kind = ExprKind::from(Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            });
            expr = kind.into_expr(span)
        }
        Ok(expr)
    }};
}

// The actual parser implementation.
//
// # Grammar:
//
// ```none
// expression -> equality
// equality   -> comparison ( ( "==" | "!=" ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term       -> factor ( ( "+" | "-" ) factor )* ;
// factor     -> unary ( ( "*" | "/" ) unary )* ;
// unary      -> ( "!" | "-" ) unary | primary ;
// primary    -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
// ```
//
// Each production has a correspondent method in the following implementation.
impl Parser<'_> {
    pub fn parse(&mut self) -> Result<Expr> {
        // TODO: Handle errors.
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr> {
        binary_expression!(
            self,
            kinds = EqualEqual | BangEqual,
            next_production = parse_comparison
        )
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        binary_expression!(
            self,
            kinds = Greater | GreaterEqual | Less | LessEqual,
            next_production = parse_term
        )
    }

    fn parse_term(&mut self) -> Result<Expr> {
        binary_expression!(
            self,
            kinds = Plus | Minus, //↵
            next_production = parse_factor
        )
    }

    fn parse_factor(&mut self) -> Result<Expr> {
        binary_expression!(
            self,
            kinds = Star | Slash, //↵
            next_production = parse_unary
        )
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        use TokenKind::{Bang, Minus};
        if let Bang | Minus = self.current_token.kind {
            let operator = self.advance().clone();
            let operand = self.parse_unary()?;
            let span = operator.span.to(operand.span);
            return Ok(Expr {
                kind: ExprKind::from(Unary {
                    operator,
                    operand: operand.into(),
                }),
                span,
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        use TokenKind::*;
        match self.current_token.kind {
            String(_) | Number(_) | True | False | Nil => {
                let token = self.advance();
                Ok(Expr {
                    kind: ExprKind::from(Literal::from(token.kind)),
                    span: token.span,
                })
            }
            LeftParen => {
                let left_paren = self.advance();
                let expr = self.parse_expression()?.into();
                match self.current_token.kind {
                    RightParen => {
                        let right_paren = self.advance();
                        Ok(Expr {
                            kind: ExprKind::from(Group { expr }),
                            span: left_paren.span.to(right_paren.span),
                        })
                    }
                    ref unexpected => {
                        bail!("Expected `)` after expression, found `{:?}`.", unexpected)
                    }
                }
            }
            ref unexpected => bail!("Expected expression, found `{:?}`.", unexpected),
        }
    }
}

// The parser helper methods.
impl<'src> Parser<'src> {
    /// Creates a new parser.
    pub fn new(scanner: Scanner<'src>) -> Self {
        let mut scanner = scanner.peekable();
        let current_token = scanner.next().unwrap();
        Self {
            scanner,
            current_token,
        }
    }

    /// Checks if the given token kind shall be ignored by this parser.
    #[inline]
    fn is_ignored_kind(kind: &TokenKind) -> bool {
        use TokenKind::*;
        match kind {
            Comment(_) | NewLine | Whitespace => true,
            _ => false,
        }
    }

    /// Advances the `current_token` field and returns the previous one.
    #[inline]
    fn advance(&mut self) -> Token {
        let prev = self.current_token.clone();
        loop {
            self.current_token = self.scanner.next().unwrap_or_else(|| {
                assert_eq!(prev.kind, TokenKind::Eof);
                prev.clone()
            });
            if !Self::is_ignored_kind(&self.current_token.kind) {
                break;
            }
        }
        prev
    }
}
