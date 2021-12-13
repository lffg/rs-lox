use anyhow::{bail, Result};

use crate::{
    ast::expr::{Binary, BinaryOperator, Expr, Group, Literal, Unary, UnaryOperator},
    token::{Token, TokenKind},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! binary_production {
    ($self:expr, next = $next:ident, token_kinds = $( $kind:ident )|+ ) => {{
        let mut expr = $self.$next()?;
        while let $( TokenKind::$kind )|+ = $self.current_kind() {
            let operator: BinaryOperator = $self.advance_kind().clone().into();
            expr = Binary {
                left: expr.into(),
                operator,
                right: $self.$next()?.into(),
            }
            .into();
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
impl Parser {
    pub fn parse(&mut self) -> Result<Expr> {
        let expr = self.expression()?;
        match self.current_kind() {
            TokenKind::Eof => Ok(expr),
            unexpected => bail!("Expected `eof`, found `{:?}`.", unexpected),
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        binary_production!(
            self,
            next = comparison,
            token_kinds = BangEqual | EqualEqual
        )
    }

    fn comparison(&mut self) -> Result<Expr> {
        binary_production!(
            self,
            next = term,
            token_kinds = Greater | GreaterEqual | Less | LessEqual
        )
    }

    fn term(&mut self) -> Result<Expr> {
        binary_production!(self, next = factor, token_kinds = Plus | Minus)
    }

    fn factor(&mut self) -> Result<Expr> {
        binary_production!(self, next = unary, token_kinds = Star | Slash)
    }

    fn unary(&mut self) -> Result<Expr> {
        use TokenKind::{Bang, Minus};
        if let Bang | Minus = self.current_kind() {
            let operator: UnaryOperator = self.advance_kind().clone().into();
            let operand = self.unary()?.into();
            let unary = Unary { operator, operand };
            return Ok(unary.into());
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        use TokenKind::*;
        match self.current_kind() {
            String(_) | Number(_) | True | False | Nil => {
                let literal: Literal = self.advance_kind().clone().into();
                Ok(literal.into())
            }
            LeftParen => {
                self.advance(); // Consumes `(`
                let inner = self.expression()?.into();
                match self.current_kind() {
                    RightParen => {
                        self.advance(); // Consumes `)`
                        Ok(Group { inner }.into())
                    }
                    unexpected => bail!("Expected `)` after expression, found `{:?}`.", unexpected),
                }
            }
            unexpected => bail!("Expected expression, found `{:?}`.", unexpected),
        }
    }
}

// The scanner helper methods.
impl Parser {
    /// Creates a new parser.
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current = 0;
        Parser { tokens, current }
    }

    /// Returns the current `Token`.
    fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns the current `TokenKind`.
    fn current_kind(&self) -> &TokenKind {
        &self.current().kind
    }

    /// Advances the current parser token. Returns the previous `Token`.
    fn advance(&mut self) -> &Token {
        let old = self.current;
        if !self.finished() {
            self.current += 1;
        }
        &self.tokens[old]
    }

    /// Advances the current parser token. Returns the previous `TokenKind`.
    fn advance_kind(&mut self) -> &TokenKind {
        &self.advance().kind
    }

    /// Checks if the parser has finished.
    fn finished(&self) -> bool {
        *self.current_kind() == TokenKind::Eof
    }
}
