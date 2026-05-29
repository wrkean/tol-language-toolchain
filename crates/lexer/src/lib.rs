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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind::*;

    fn lex(input: &str) -> Vec<Token<'_>> {
        Lexer::new(input).run()
    }

    #[test]
    fn lexes_keywords_identifiers_literals_and_punctuation() {
        let tokens = lex("par halaga = 12_345; ang x: 3.14");

        let kinds: Vec<_> = tokens.iter().map(Token::kind).collect();
        assert_eq!(
            kinds,
            vec![
                &Par,
                &Identifier,
                &Equal,
                &IntLiteral,
                &SemiColon,
                &Ang,
                &Identifier,
                &Colon,
                &FloatLiteral,
            ]
        );

        let lexemes: Vec<_> = tokens.iter().map(Token::lexeme).collect();
        assert_eq!(
            lexemes,
            vec!["par", "halaga", "=", "12_345", ";", "ang", "x", ":", "3.14"]
        );
    }

    #[test]
    fn tracks_byte_spans_for_tokens() {
        let tokens = lex("par x\n  y");

        let spans: Vec<_> = tokens.iter().map(|token| token.span().clone()).collect();
        assert_eq!(spans, vec![0..3, 4..5, 5..6, 8..9]);

        let lexemes: Vec<_> = tokens.iter().map(Token::lexeme).collect();
        assert_eq!(lexemes, vec!["par", "x", ";", "y"]);
    }

    #[test]
    fn skips_whitespace_and_line_comments() {
        let tokens = lex("  par\tfoo -- ignored until newline\nang");

        let kinds: Vec<_> = tokens.iter().map(Token::kind).collect();
        assert_eq!(kinds, vec![&Par, &Identifier, &SemiColon, &Ang]);

        let lexemes: Vec<_> = tokens.iter().map(Token::lexeme).collect();
        assert_eq!(lexemes, vec!["par", "foo", ";", "ang"]);
    }

    #[test]
    fn infers_semicolon_after_expression_ending_tokens() {
        let tokens = lex("foo\n123\n4.5\n(a)\n[b]");

        let kinds: Vec<_> = tokens.iter().map(Token::kind).collect();
        assert_eq!(
            kinds,
            vec![
                &Identifier,
                &SemiColon,
                &IntLiteral,
                &SemiColon,
                &FloatLiteral,
                &SemiColon,
                &LeftParen,
                &Identifier,
                &RightParen,
                &SemiColon,
                &LeftSquare,
                &Identifier,
                &RightSquare,
            ]
        );
    }

    #[test]
    fn does_not_infer_semicolon_inside_parentheses_or_square_brackets() {
        let tokens = lex("(foo\nbar)\n[baz\nqux]");

        let kinds: Vec<_> = tokens.iter().map(Token::kind).collect();
        assert_eq!(
            kinds,
            vec![
                &LeftParen,
                &Identifier,
                &Identifier,
                &RightParen,
                &SemiColon,
                &LeftSquare,
                &Identifier,
                &Identifier,
                &RightSquare,
            ]
        );
    }

    #[test]
    fn does_not_infer_semicolon_after_non_expression_tokens_or_leading_newline() {
        let tokens = lex("\npar\n=\n+\n{\n}");

        let kinds: Vec<_> = tokens.iter().map(Token::kind).collect();
        assert_eq!(kinds, vec![&Par, &Equal, &Plus, &LeftBrace, &RightBrace]);
    }

    #[test]
    fn lexes_all_single_character_operators_and_delimiters() {
        let tokens = lex("(){}[],=:;+-*/");

        let kinds: Vec<_> = tokens.iter().map(Token::kind).collect();
        assert_eq!(
            kinds,
            vec![
                &LeftParen,
                &RightParen,
                &LeftBrace,
                &RightBrace,
                &LeftSquare,
                &RightSquare,
                &Comma,
                &Equal,
                &Colon,
                &SemiColon,
                &Plus,
                &Minus,
                &Star,
                &Slash,
            ]
        );
    }
}
