#![allow(unused)]

use std::mem;

use logos::Logos;
use shared::Span;

use crate::token::{Token, TokenKind};

pub mod token;

pub struct Lexer<'src> {
    input: &'src str,
    tokens: Vec<Token<'src>>,
    bracket_depth: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input,
            tokens: Vec::new(),
            bracket_depth: 0,
        }
    }

    pub fn run(&mut self) -> Vec<Token<'src>> {
        let mut lex = TokenKind::lexer(self.input);
        while let Some(result) = lex.next() {
            match result {
                Ok(tokenkind) => self.lex_token(lex.slice(), tokenkind, lex.span()),
                Err(_) => panic!("Some error occured"),
            }
        }

        mem::take(&mut self.tokens)
    }

    fn lex_token(&mut self, slice: &'src str, tokenkind: TokenKind, span: Span) {
        match tokenkind {
            TokenKind::LeftParen | TokenKind::LeftSquare => {
                self.bracket_depth += 1;
                self.add_token(slice, tokenkind, span);
            }
            TokenKind::RightParen | TokenKind::RightSquare => {
                self.bracket_depth -= 1;
                self.add_token(slice, tokenkind, span);
            }
            TokenKind::Newline => self.handle_semicolon_inferring(slice, tokenkind, span),
            _ => self.add_token(slice, tokenkind, span),
        }
    }

    fn add_token(&mut self, slice: &'src str, tokenkind: TokenKind, span: Span) {
        self.tokens.push(Token::new(slice, tokenkind, span));
    }

    fn handle_semicolon_inferring(&mut self, slice: &'src str, tokenkind: TokenKind, span: Span) {
        let Some(last_token) = self.tokens.last() else {
            return;
        };

        if last_token.kind().infers_semicolon() && self.bracket_depth == 0 {
            self.add_token(";", TokenKind::SemiColon, span);
        }
    }
}
