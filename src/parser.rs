use std::{iter::Peekable, mem};

use crate::{
    ast::expr::{Binary, Expr, ExprKind, Group, Literal, Unary},
    parser::error::{PResult, ParseError},
    scanner::Scanner,
    token::{Token, TokenKind},
};

mod error;

macro_rules! binary_expression {
    ($self:expr, kinds = $( $kind:ident )|+, next_production = $next:ident) => {{
        let mut expr = $self.$next()?;
        while let $( TokenKind::$kind )|+ = $self.current_token.kind {
            let operator = $self.advance()?.clone();
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

pub struct Parser<'src> {
    scanner: Peekable<Scanner<'src>>,
    current_token: Token,
    prev_token: Token,
    diagnostics: Vec<ParseError>,
}

// The parser implementation.
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
    pub fn parse(mut self) -> (Option<Expr>, Vec<ParseError>) {
        // The first advancement:
        if let Err(err) = self.advance() {
            self.synchronize_with(err);
        }

        // THIS IS TEMPORARY, WILL BE REMOVED SOON.
        let expr = match self.temp_parse_finished_expression() {
            Ok(expr) => Some(expr),
            Err(err) => {
                self.synchronize_with(err);
                assert_eq!(self.current_token.kind, TokenKind::Eof);
                None
            }
        };
        (expr, self.diagnostics)
    }

    // THIS IS TEMPORARY, WILL BE REMOVED SOON.
    fn temp_parse_finished_expression(&mut self) -> PResult<Expr> {
        let expr = self.parse_expression()?;
        match self.current_token.kind {
            TokenKind::Eof => Ok(expr),
            _ => Err(ParseError::UnexpectedToken {
                message: "Expected end of input".into(),
                offending: self.current_token.clone(),
                expected: Some(TokenKind::Eof),
            }),
        }
    }

    fn parse_expression(&mut self) -> PResult<Expr> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> PResult<Expr> {
        binary_expression!(
            self,
            kinds = EqualEqual | BangEqual,
            next_production = parse_comparison
        )
    }

    fn parse_comparison(&mut self) -> PResult<Expr> {
        binary_expression!(
            self,
            kinds = Greater | GreaterEqual | Less | LessEqual,
            next_production = parse_term
        )
    }

    fn parse_term(&mut self) -> PResult<Expr> {
        binary_expression!(
            self, //↵
            kinds = Plus | Minus,
            next_production = parse_factor
        )
    }

    fn parse_factor(&mut self) -> PResult<Expr> {
        binary_expression!(
            self, //↵
            kinds = Star | Slash,
            next_production = parse_unary
        )
    }

    fn parse_unary(&mut self) -> PResult<Expr> {
        use TokenKind::{Bang, Minus};
        if let Bang | Minus = self.current_token.kind {
            let operator = self.advance()?.clone();
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

    fn parse_primary(&mut self) -> PResult<Expr> {
        use TokenKind::*;
        match self.current_token.kind {
            String(_) | Number(_) | True | False | Nil => {
                let token = self.advance()?;
                Ok(Expr {
                    kind: ExprKind::from(Literal::from(token.kind.clone())),
                    span: token.span,
                })
            }
            LeftParen => {
                let left_paren_span = self.advance()?.span;
                let expr = self.parse_expression()?.into();
                match self.current_token.kind {
                    RightParen => {
                        let right_paren_span = self.advance()?.span;
                        Ok(Expr {
                            kind: ExprKind::from(Group { expr }),
                            span: left_paren_span.to(right_paren_span),
                        })
                    }
                    _ => Err(ParseError::UnexpectedToken {
                        message: "Expected group to be closed".into(),
                        offending: self.current_token.clone(),
                        expected: Some(TokenKind::RightParen),
                    }),
                }
            }
            _ => Err(ParseError::UnexpectedToken {
                message: "Expected any expression".into(),
                offending: self.current_token.clone(),
                expected: None,
            }),
        }
    }
}

// The parser helper methods.
impl<'src> Parser<'src> {
    /// Creates a new parser.
    pub fn new(scanner: Scanner<'src>) -> Self {
        Self {
            scanner: scanner.peekable(),
            current_token: Token::dummy(),
            prev_token: Token::dummy(),
            diagnostics: Vec::new(),
        }
    }

    /// Checks if the given token kind shall be ignored by this parser.
    #[inline]
    fn is_ignored_kind(kind: &TokenKind) -> bool {
        use TokenKind::*;
        matches!(kind, Comment(_) | NewLine | Whitespace)
    }

    /// Advances the parser and returns a reference to the `prev_token` field.
    fn advance(&mut self) -> PResult<&Token> {
        self.advance_unchecked();
        // Handle errors from scanner:
        if let TokenKind::Error(message) = &self.current_token.kind {
            return Err(ParseError::ScannerError {
                message: message.clone(),
                offending_span: self.current_token.span,
            });
        }
        Ok(&self.prev_token)
    }

    /// Advances the parser without checking for an `Error` token.
    fn advance_unchecked(&mut self) {
        let next = loop {
            let maybe_next = self.scanner.next().expect("Cannot advance past Eof.");
            if !Self::is_ignored_kind(&maybe_next.kind) {
                break maybe_next;
            }
        };
        self.prev_token = mem::replace(&mut self.current_token, next);
    }

    /// Synchronizes the parser state with the current token.
    ///
    /// A synchronization is needed in order to match the parser state to the current token.
    ///
    /// When an error is encountered in a `parse_` method, a `ParseError` is returned. These kind of
    /// errors are forwarded using the `?` operator, which, in practice, unwinds the parser stack
    /// frame. The question mark operator shall not be used in synchronization points.
    /// Such synchronization points will call this method, `synchronize_with`.
    ///
    /// The synchronization process discards all tokens until it reaches a grammar rule which marks
    /// a synchronization point.
    ///
    /// In this implementation, synchronization is done in statement boundaries:
    ///   * If the previous token is a semicolon, the parser is *probably* (exceptions exists, such
    ///     as a semicolon in a for loop) starting a new statement.
    ///   * If the next token is a listed (in the implementation) keyword the parser is also
    ///     starting a new statement.
    fn synchronize_with(&mut self, error: ParseError) {
        self.diagnostics.push(error);

        // If the end was already reached there is no need to advance the parser.
        if self.is_at_end() {
            return;
        }

        self.advance_unchecked();
        use TokenKind::*;
        while !{
            self.is_at_end()
                || matches!(self.prev_token.kind, Semicolon)
                || matches!(
                    self.current_token.kind,
                    Class | For | Fun | If | Print | Return | Var | While
                )
        } {
            self.advance_unchecked();
        }
    }

    /// Checks if the parser has finished.
    #[inline]
    fn is_at_end(&self) -> bool {
        self.current_token.kind == TokenKind::Eof
    }
}
