use std::{borrow::Borrow, iter::Peekable, mem};

use crate::{
    ast::{
        expr::{self, Expr, ExprKind},
        stmt::{self, Stmt},
    },
    data::{LoxIdent, LoxValue},
    parser::{error::ParseError, scanner::Scanner, state::ParserOptions},
    span::Span,
    token::{Token, TokenKind},
};

pub mod error;
pub mod scanner;
pub mod state;

/// Parse result
type PResult<T> = Result<T, ParseError>;

pub type ParserOutcome = (Vec<Stmt>, Vec<ParseError>);

pub struct Parser<'src> {
    scanner: Peekable<Scanner<'src>>,
    current_token: Token,
    prev_token: Token,
    diagnostics: Vec<ParseError>,
    pub options: ParserOptions,
}

// The parser implementation.
//
// # Grammar:
//
// -----------------------------------------------------------------------------
//
// program       ::= decl* EOF ;
//
// decl          ::= var_decl
//                 | fun_decl
//                 | stmt ;
//
// var_decl      ::= "var" IDENTIFIER ( "=" expr )? ";" ;
// class_decl    ::= "class" IDENTIFIER "{" fn* "}" ;
// fun_decl      ::= "fun" fn ;
//
// fn            ::= IDENTIFIER "(" params? ")" block_stmt ;
// params        ::= IDENTIFIER ( "," IDENTIFIER )* ;
//
// stmt          ::= if_stmt
//                 | for_stmt
//                 | while_stmt
//                 | return_stmt
//                 | print_stmt
//                 | block_stmt
//                 | expr_stmt ;
//
// if_stmt       ::= "if" "(" expr ")" statement ( "else" statement )? ;
// for_stmt      ::= "for"
//                   "(" ( var_decl | expr_stmt | ";" ) expr? ";" expr? ")"
//                   statement ;
// while_stmt    ::= "while" "(" expr ")" statement ;
// return_stmt   ::= "return" expr? ";" ;
// print_stmt    ::= "print" expr ";" ;
// block_stmt    ::= "{" declaration* "}" ;
// expr_stmt     ::= expr ";" ;
//
// expr          ::= assignment ;
// assignment    ::= ( call_or_get "." )? IDENTIFIER "=" assignment
//                 | logic_or ;
// logic_or      ::= logic_and ( "or" logic_and )* ;
// logic_and     ::= equality ( "and" equality )* ;
// equality      ::= comparison ( ( "==" | "!=" ) comparison )* ;
// comparison    ::= term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term          ::= factor ( ( "+" | "-" ) factor )* ;
// factor        ::= unary ( ( "*" | "/" ) unary )* ;
// unary         ::= ( "show" | "typeof" | "!" | "-" ) unary
//                 | call_or_get ;
// call_or_get ::= primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
// arguments     ::= expr ( "," expr )* ;
// primary       ::= IDENTIFIER
//                 | NUMBER | STRING
//                 | "true" | "false"
//                 | "nil"
//                 | "(" expr ")" ;
//
// -----------------------------------------------------------------------------
//
// Each production has a correspondent method in the following implementation.
impl Parser<'_> {
    pub fn parse(mut self) -> ParserOutcome {
        (self.parse_program(), self.diagnostics)
    }

    fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.parse_decl());
        }
        stmts
    }

    //
    // Declarations
    //

    fn parse_decl(&mut self) -> Stmt {
        use TokenKind::*;
        let result = match self.current_token.kind {
            Var => self.parse_var_decl(),
            Class => self.parse_class_decl(),
            Fun => self.parse_fun_decl(),
            _ => self.parse_stmt(),
        };

        match result {
            Ok(stmt) => stmt,
            Err(error) => {
                self.diagnostics.push(error);
                self.synchronize();
                let lo = self.current_token.span.lo;
                Stmt::new(Span::new(lo, lo), stmt::Dummy())
            }
        }
    }

    fn parse_var_decl(&mut self) -> PResult<Stmt> {
        use TokenKind::*;
        let var_span = self.consume(Var, S_MUST)?.span;

        let name = self.consume_ident("Expected variable name")?;
        let init = self.take(Equal).then(|| self.parse_expr()).transpose()?;

        let semicolon_span = self
            .consume(Semicolon, "Expected `;` after variable declaration")?
            .span;

        Ok(Stmt::new(
            var_span.to(semicolon_span),
            stmt::VarDecl { name, init },
        ))
    }

    fn parse_class_decl(&mut self) -> PResult<Stmt> {
        use TokenKind::*;
        let class_span = self.consume(Class, S_MUST)?.span;

        let name = self.consume_ident("Expected class name")?;

        let (methods, class_body_span) = self.paired_spanned(
            LeftBrace,
            "Expected `{` before class body",
            "Expected `}` after class body",
            |this| {
                let mut methods = Vec::new();
                while !this.is(RightBrace) && !this.is_at_end() {
                    methods.push(this.parse_fn_params_and_body("method")?);
                }
                Ok(methods)
            },
        )?;

        Ok(Stmt::new(
            class_span.to(class_body_span),
            stmt::ClassDecl { name, methods },
        ))
    }

    fn parse_fun_decl(&mut self) -> PResult<Stmt> {
        use TokenKind::*;
        let fun_span = self.consume(Fun, S_MUST)?.span;
        let fun = self.parse_fn_params_and_body("function")?;
        Ok(Stmt::new(fun_span.to(fun.span), fun))
    }

    fn parse_fn_params_and_body(&mut self, kind: &'static str) -> PResult<stmt::FunDecl> {
        use TokenKind::*;
        let name = self.consume_ident(format!("Expected {} name", kind))?;

        let params = self.paired(
            LeftParen,
            format!("Expected `(` after {} name", kind),
            format!("Expected `)` after {} parameter list", kind),
            |this| {
                let mut params = Vec::new();
                if !this.is(RightParen) {
                    loop {
                        let param = this.consume_ident("Expected parameter name")?;
                        params.push(param);
                        if !this.take(Comma) {
                            break;
                        }
                    }
                }
                Ok(params)
            },
        )?;

        let (body, body_span) = self.parse_block()?;
        Ok(stmt::FunDecl {
            span: name.span.to(body_span),
            name,
            params,
            body,
        })
    }

    //
    // Statements
    //

    fn parse_stmt(&mut self) -> PResult<Stmt> {
        use TokenKind::*;
        match self.current_token.kind {
            If => self.parse_if_stmt(),
            For => self.parse_for_stmt(),
            While => self.parse_while_stmt(),
            Return => self.parse_return_stmt(),
            Print => self.parse_print_stmt(),
            LeftBrace => {
                let (stmts, span) = self.parse_block()?;
                Ok(Stmt::new(span, stmt::Block { stmts }))
            }
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_if_stmt(&mut self) -> PResult<Stmt> {
        use TokenKind::*;
        let if_token_span = self.consume(If, S_MUST)?.span;

        let cond = self.paired(
            LeftParen,
            "Expected `if` condition group opening",
            "Expected `if` condition group to be closed",
            |this| this.parse_expr(),
        )?;
        let then_branch = self.parse_stmt()?;
        let else_branch = self.take(Else).then(|| self.parse_stmt()).transpose()?;

        Ok(Stmt::new(
            if_token_span.to(else_branch
                .as_ref()
                .map(|it| it.span)
                .unwrap_or(then_branch.span)),
            stmt::If {
                cond,
                then_branch: then_branch.into(),
                else_branch: else_branch.map(|it| it.into()),
            },
        ))
    }

    // In this implementation, all `for` statements are translated to `while` statements by the
    // parser. Hence there is not even a `StmtKind::For` kind since it is a syntactic sugar. E.g.:
    //
    // ```
    // for (var i = 1; i <= 10; i = i + 1) { print show i; }
    // ```
    //
    // Is translated to:
    //
    // ```
    // {
    //    var i = 1;
    //    while (i <= 10) {
    //      { print show i; }
    //      i = i + 1;
    //    }
    // }
    // ```
    fn parse_for_stmt(&mut self) -> PResult<Stmt> {
        use TokenKind::*;
        let for_token_span = self.consume(For, S_MUST)?.span;

        let (init, cond, incr) = self.paired(
            LeftParen,
            "Expected `for` clauses group opening",
            "Expected `for` clauses group to be closed",
            |this| {
                let init = match this.current_token.kind {
                    Semicolon => {
                        this.advance();
                        None
                    }
                    Var => Some(this.parse_var_decl()?),
                    _ => Some(this.parse_expr_stmt()?),
                };
                let cond = match this.current_token.kind {
                    // If there is none condition in the for clauses, the parser creates a synthetic `true`
                    // literal expression. This must be placed here to capture the current span (†).
                    Semicolon => Expr::new(
                        {
                            let lo = this.current_token.span.lo; // <--- This span. (†)
                            Span::new(lo, lo)
                        },
                        expr::Lit {
                            value: LoxValue::Boolean(true),
                        },
                    ),
                    _ => this.parse_expr()?,
                };
                this.consume(Semicolon, "Expected `;` after `for` condition")?;
                let incr = match this.current_token.kind {
                    RightParen => None,
                    _ => Some(this.parse_expr()?),
                };
                Ok((init, cond, incr))
            },
        )?;
        let mut body = self.parse_stmt()?;

        // Desugar `for` increment:
        if let Some(incr) = incr {
            body = Stmt::new(
                body.span,
                stmt::Block {
                    stmts: Vec::from([body, Stmt::new(incr.span, stmt::Expr { expr: incr })]),
                },
            );
        }

        // Create the while:
        body = Stmt::new(
            for_token_span.to(body.span),
            stmt::While {
                cond,
                body: body.into(),
            },
        );

        // Desugar `for` initializer:
        if let Some(init) = init {
            body = Stmt::new(
                body.span,
                stmt::Block {
                    stmts: Vec::from([init, body]),
                },
            );
        }

        Ok(body)
    }

    fn parse_while_stmt(&mut self) -> PResult<Stmt> {
        use TokenKind::*;
        let while_token_span = self.consume(While, S_MUST)?.span;

        let cond = self.paired(
            LeftParen,
            "Expected `while` condition group opening",
            "Expected `while` condition group to be closed",
            |this| this.parse_expr(),
        )?;
        let body = self.parse_stmt()?;

        Ok(Stmt::new(
            while_token_span.to(body.span),
            stmt::While {
                cond,
                body: body.into(),
            },
        ))
    }

    fn parse_return_stmt(&mut self) -> PResult<Stmt> {
        let return_span = self.consume(TokenKind::Return, S_MUST)?.span;

        let value = (!self.is(TokenKind::Semicolon))
            .then(|| self.parse_expr())
            .transpose()?;

        let semicolon_span = self
            .consume(TokenKind::Semicolon, "Expected `;` after return")?
            .span;

        Ok(Stmt::new(
            return_span.to(semicolon_span),
            stmt::Return { value, return_span },
        ))
    }

    fn parse_print_stmt(&mut self) -> PResult<Stmt> {
        let print_token_span = self.consume(TokenKind::Print, S_MUST)?.span;

        let expr = self.parse_expr()?;
        let semicolon_span = self
            .consume(TokenKind::Semicolon, "Expected `;` after value")?
            .span;

        Ok(Stmt::new(
            print_token_span.to(semicolon_span),
            stmt::Print { expr, debug: false },
        ))
    }

    fn parse_block(&mut self) -> PResult<(Vec<Stmt>, Span)> {
        self.paired_spanned(
            TokenKind::LeftBrace,
            "Expected block to be opened",
            "Expected block to be closed",
            |this| {
                let mut stmts = Vec::new();
                while !this.is(TokenKind::RightBrace) && !this.is_at_end() {
                    stmts.push(this.parse_decl());
                }
                Ok(stmts)
            },
        )
    }

    fn parse_expr_stmt(&mut self) -> PResult<Stmt> {
        let expr = self.parse_expr()?;

        // If the parser is running in the REPL mode and the next token is of kind `Eof`, it will
        // emit a `Print` statement in order to show the given expression's value.
        if self.options.repl_mode && self.is_at_end() {
            return Ok(Stmt::new(expr.span, stmt::Print { expr, debug: true }));
        }

        let semicolon_span = self
            .consume(TokenKind::Semicolon, "Expected `;` after expression")?
            .span;

        Ok(Stmt::new(expr.span.to(semicolon_span), stmt::Expr { expr }))
    }

    //
    // Expressions
    //

    fn parse_expr(&mut self) -> PResult<Expr> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> PResult<Expr> {
        // The parser does not yet know if `left` should be used as an expression (i.e. an rvalue)
        // or as an "assignment target" (i.e. an lvalue).
        let left = self.parse_or()?;

        if self.take(TokenKind::Equal) {
            // Since assignments are right associative, we use right recursion to parse its value.
            // The right-most assignment value should be evaluated first (down in the parse tree),
            // so it should be parsed last. This semantic can be coded with this kind of recursion.
            let value = self.parse_assignment()?;
            let span = left.span.to(value.span);

            // Now the parser knows that `left` must be an lvalue, otherwise it is an error.
            match left.kind {
                ExprKind::Var(expr::Var { name }) => Ok(Expr::new(
                    span,
                    expr::Assignment {
                        name,
                        value: value.into(),
                    },
                )),
                ExprKind::Get(expr::Get { name, object }) => Ok(Expr::new(
                    span,
                    expr::Set {
                        object,
                        name,
                        value: value.into(),
                    },
                )),
                _ => Err(ParseError::Error {
                    message: "Invalid assignment target".into(),
                    span: left.span,
                }),
            }
        } else {
            Ok(left)
        }
    }

    fn parse_or(&mut self) -> PResult<Expr> {
        bin_expr!(
            self,
            parse_as = Logical,
            token_kinds = Or,
            next_production = parse_and
        )
    }

    fn parse_and(&mut self) -> PResult<Expr> {
        bin_expr!(
            self,
            parse_as = Logical,
            token_kinds = And,
            next_production = parse_equality
        )
    }

    fn parse_equality(&mut self) -> PResult<Expr> {
        bin_expr!(
            self,
            parse_as = Binary,
            token_kinds = EqualEqual | BangEqual,
            next_production = parse_comparison
        )
    }

    fn parse_comparison(&mut self) -> PResult<Expr> {
        bin_expr!(
            self,
            parse_as = Binary,
            token_kinds = Greater | GreaterEqual | Less | LessEqual,
            next_production = parse_term
        )
    }

    fn parse_term(&mut self) -> PResult<Expr> {
        bin_expr!(
            self,
            parse_as = Binary,
            token_kinds = Plus | Minus,
            next_production = parse_factor
        )
    }

    fn parse_factor(&mut self) -> PResult<Expr> {
        bin_expr!(
            self,
            parse_as = Binary,
            token_kinds = Star | Slash,
            next_production = parse_unary
        )
    }

    fn parse_unary(&mut self) -> PResult<Expr> {
        use TokenKind::*;
        if let Bang | Minus | Typeof | Show = self.current_token.kind {
            let operator = self.advance().clone();
            let operand = self.parse_unary()?;
            return Ok(Expr::new(
                operator.span.to(operand.span),
                expr::Unary {
                    operator,
                    operand: operand.into(),
                },
            ));
        }
        self.parse_call_or_get()
    }

    fn parse_call_or_get(&mut self) -> PResult<Expr> {
        use TokenKind::*;
        let mut expr = self.parse_primary()?;

        loop {
            expr = match self.current_token.kind {
                LeftParen => self.finish_call_parsing(expr)?,
                Dot => {
                    self.advance(); // Consumes the `.`
                    let name = self.consume_ident("Expect property name after `.`")?;
                    let span = expr.span.to(name.span);
                    let object = expr.into();
                    Expr::new(span, expr::Get { object, name })
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    fn finish_call_parsing(&mut self, curr_expr: Expr) -> PResult<Expr> {
        use TokenKind::*;
        let (args, call_span) = self.paired_spanned(
            LeftParen,
            S_MUST,
            "Expected `)` to close call syntax",
            |this| {
                let mut args = Vec::new();
                if !this.is(RightParen) {
                    loop {
                        args.push(this.parse_expr()?);
                        if !this.take(Comma) {
                            break;
                        }
                    }
                }
                Ok(args)
            },
        )?;

        if args.len() >= 255 {
            self.diagnostics.push(ParseError::Error {
                message: "Call can't have more than 255 arguments".into(),
                span: call_span,
            })
        }

        Ok(Expr::new(
            curr_expr.span.to(call_span),
            expr::Call {
                callee: curr_expr.into(),
                args,
            },
        ))
    }

    fn parse_primary(&mut self) -> PResult<Expr> {
        use TokenKind::*;
        match &self.current_token.kind {
            String(_) | Number(_) | True | False | Nil => {
                let token = self.advance();
                Ok(Expr::new(token.span, expr::Lit::from(token.clone())))
            }
            Identifier(_) => {
                let name = self.consume_ident(S_MUST)?;
                Ok(Expr::new(name.span, expr::Var { name }))
            }
            LeftParen => {
                let (expr, span) = self.paired_spanned(
                    LeftParen,
                    S_MUST,
                    "Expected group to be closed",
                    |this| this.parse_expr(),
                )?;
                Ok(Expr::new(span, expr::Group { expr: expr.into() }))
            }
            _ => Err(self.unexpected("Expected any expression", None)),
        }
    }
}

// The parser helper methods.
impl<'src> Parser<'src> {
    /// Creates a new parser.
    pub fn new(src: &'src str) -> Self {
        let mut parser = Self {
            scanner: Scanner::new(src).peekable(),
            current_token: Token::dummy(),
            prev_token: Token::dummy(),
            diagnostics: Vec::new(),
            options: ParserOptions::default(),
        };
        parser.advance(); // The first advancement.
        parser
    }

    /// Advances the parser and returns a reference to the `prev_token` field.
    fn advance(&mut self) -> &Token {
        let next = loop {
            let maybe_next = self.scanner.next().expect("Cannot advance past Eof.");
            // Report and ignore tokens with the `Error` kind:
            if let TokenKind::Error(error) = maybe_next.kind {
                self.diagnostics.push(ParseError::ScanError {
                    error,
                    span: maybe_next.span,
                });
                continue;
            }
            // Handle other common ignored kinds:
            if let TokenKind::Comment(_) | TokenKind::Whitespace(_) = maybe_next.kind {
                continue;
            }
            break maybe_next;
        };
        self.prev_token = mem::replace(&mut self.current_token, next);
        &self.prev_token
    }

    /// Checks if the current token matches the kind of the given one.
    #[inline]
    fn is(&mut self, expected: impl Borrow<TokenKind>) -> bool {
        mem::discriminant(&self.current_token.kind) == mem::discriminant(expected.borrow())
    }

    /// Checks if the current token matches the kind of the given one. In such case advances and
    /// returns true. Otherwise returns false.
    fn take(&mut self, expected: TokenKind) -> bool {
        let res = self.is(expected);
        if res {
            self.advance();
        }
        res
    }

    /// Checks if the current token matches the kind of the given one. In such case advances and
    /// returns `Ok(_)` with the consumed token. Otherwise returns an expectation error with the
    /// provided message.
    fn consume(&mut self, expected: TokenKind, msg: impl Into<String>) -> PResult<&Token> {
        if self.is(&expected) {
            Ok(self.advance())
        } else {
            Err(self.unexpected(msg, Some(expected)))
        }
    }

    /// Checks if the current token is an identifier. In such case advances and returns `Ok(_)` with
    /// the parsed identifier. Otherwise returns an expectation error with the provided message.
    fn consume_ident(&mut self, msg: impl Into<String>) -> PResult<LoxIdent> {
        let expected = TokenKind::Identifier("<ident>".into());
        if self.is(&expected) {
            Ok(LoxIdent::from(self.advance().clone()))
        } else {
            Err(self.unexpected(msg, Some(expected)))
        }
    }

    /// Pair invariant.
    fn paired<I, R>(
        &mut self,
        delim_start: TokenKind,
        delim_start_expectation: impl Into<String>,
        delim_end_expectation: impl Into<String>,
        inner: I,
    ) -> PResult<R>
    where
        I: FnOnce(&mut Self) -> PResult<R>,
    {
        self.paired_spanned(
            delim_start,
            delim_start_expectation,
            delim_end_expectation,
            inner,
        )
        .map(|(ret, _)| ret)
    }

    /// Pair invariant (2), also returning the full span.
    fn paired_spanned<I, R>(
        &mut self,
        delim_start: TokenKind,
        delim_start_expectation: impl Into<String>,
        delim_end_expectation: impl Into<String>,
        inner: I,
    ) -> PResult<(R, Span)>
    where
        I: FnOnce(&mut Self) -> PResult<R>,
    {
        let start_span = self
            .consume(delim_start.clone(), delim_start_expectation)?
            .span;
        let ret = inner(self)?;
        let end_span = match self.consume(delim_start.get_pair(), delim_end_expectation) {
            Ok(token) => token.span,
            Err(error) => {
                return Err(error);
            }
        };
        Ok((ret, start_span.to(end_span)))
    }

    /// Returns an `ParseError::UnexpectedToken`.
    #[inline(always)]
    fn unexpected(&self, message: impl Into<String>, expected: Option<TokenKind>) -> ParseError {
        ParseError::UnexpectedToken {
            message: message.into(),
            expected,
            offending: self.current_token.clone(),
        }
    }

    /// Synchronizes the parser state with the current token.
    /// A synchronization is needed in order to match the parser state to the current token.
    ///
    /// When an error is encountered in a `parse_*` method, a `ParseError` is returned. These kind
    /// of errors are forwarded using the `?` operator, which, in practice, unwinds the parser
    /// stack frame. Hence the question mark operator should not be used in synchronization points.
    /// Such synchronization points call this method.
    ///
    /// The synchronization process discards all tokens until it reaches a grammar rule which marks
    /// a synchronization point.
    ///
    /// In this implementation, synchronizations are manually performed in statement boundaries:
    ///   * If the previous token is a semicolon, the parser is *probably* (exceptions exists, such
    ///     as a semicolon in a for loop) starting a new statement.
    ///   * If the next token marks the start of a new statement.
    ///
    /// Before synchronize one must not forget to emit the raised parse error.
    fn synchronize(&mut self) {
        use TokenKind::*;
        while !self.is_at_end() {
            match &self.current_token.kind {
                Semicolon => {
                    self.advance();
                    return;
                }
                Class | For | Fun | If | Print | Return | Var | While => {
                    return;
                }
                _ => self.advance(),
            };
        }
    }

    /// Checks if the parser has finished.
    #[inline]
    fn is_at_end(&self) -> bool {
        self.current_token.kind == TokenKind::Eof
    }
}

/// (String Must) Indicates the parser to emit a parser error (i.e. the parser is bugged) message.
const S_MUST: &str = "Parser bug. Unexpected token";

/// Parses a binary expression.
macro_rules! bin_expr {
    ($self:expr, parse_as = $ast_kind:ident, token_kinds = $( $kind:ident )|+, next_production = $next:ident) => {{
        let mut expr = $self.$next()?;
        while let $( TokenKind::$kind )|+ = $self.current_token.kind {
            let operator = $self.advance().clone();
            let right = $self.$next()?;
            expr = Expr::new(
                expr.span.to(right.span),
                expr::$ast_kind {
                    left: expr.into(),
                    operator,
                    right: right.into(),
                },
            );
        }
        Ok(expr)
    }};
}
use bin_expr;
