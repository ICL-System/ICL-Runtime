//! ICL Tokenizer — converts ICL text into token stream
//!
//! Handles: keywords, identifiers, string literals, integer/float literals,
//! ISO8601 timestamps, UUIDs, symbols (braces, colons, commas, brackets).
//! Comments (//) are discarded.

/// Token types for ICL syntax
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords (section names)
    Contract,
    Identity,
    PurposeStatement,
    DataSemantics,
    BehavioralSemantics,
    ExecutionConstraints,
    HumanMachineContract,
    Extensions,

    // Type keywords
    IntegerType,
    FloatType,
    StringType,
    BooleanType,
    Iso8601Type,
    UuidType,
    ArrayType,
    MapType,
    ObjectType,
    EnumType,

    // Literals
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    TimestampLiteral(String),
    UuidLiteral(String),

    // Symbols
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    LAngle,    // <
    RAngle,    // >
    Colon,     // :
    Comma,     // ,
    Equals,    // =

    // Other
    Identifier(String),
    Eof,
}

/// Position in source text for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

/// Token with source position
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

/// Tokenizer for ICL source text
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    /// Create a new tokenizer for the given input text
    pub fn new(text: &str) -> Self {
        Tokenizer {
            input: text.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input into a stream of spanned tokens
    pub fn tokenize(&mut self) -> crate::Result<Vec<SpannedToken>> {
        // TODO: Phase 1.1 — Implement character-by-character scanning
        // TODO: Phase 1.1 — Handle all token types
        // TODO: Phase 1.1 — Skip comments (//)
        // TODO: Phase 1.1 — Track line/column for error reporting
        Err(crate::Error::ParseError("Tokenizer not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_keywords() {
        // TODO: Test that ICL keywords are recognized
    }

    #[test]
    fn test_tokenize_string_literals() {
        // TODO: Test string literal parsing with escape sequences
    }

    #[test]
    fn test_tokenize_numbers() {
        // TODO: Test integer and float literal parsing
    }

    #[test]
    fn test_skip_comments() {
        // TODO: Test that // comments are skipped
    }

    #[test]
    fn test_tokenize_determinism() {
        // TODO: 100 iterations, identical token streams
    }
}
