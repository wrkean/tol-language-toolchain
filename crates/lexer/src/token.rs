use logos::Logos;
use shared::Span;

pub struct Token<'src> {
    lexeme: &'src str,
    kind: TokenKind,
    span: Span,
}

impl<'src> Token<'src> {
    pub fn new(lexeme: &'src str, kind: TokenKind, span: Span) -> Self {
        Self { lexeme, kind, span }
    }

    pub fn lexeme(&self) -> &'src str {
        self.lexeme
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum TokenKind {
    #[token("par")]
    Par,
    #[token("ang")]
    Ang,
    #[token("iiba")]
    Iiba,
    #[token("dapat")]
    Dapat,
    #[token("ibalik")]
    Ibalik,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,
    #[token(",")]
    Comma,
    #[token("=")]
    Equal,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("\n")]
    Newline,

    #[regex(r"[_A-Za-z][_A-Za-z0-9]*")]
    Identifier,
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*")]
    FloatLiteral,
    #[regex(r"[0-9][0-9_]*")]
    IntLiteral,

    #[regex(r"--[^\n]*", logos::skip, allow_greedy = true)]
    Comment,
}

impl TokenKind {
    pub fn infers_semicolon(&self) -> bool {
        matches!(
            self,
            TokenKind::RightParen
                | TokenKind::RightSquare
                | TokenKind::Identifier
                | TokenKind::FloatLiteral
                | TokenKind::IntLiteral
        )
    }
}
