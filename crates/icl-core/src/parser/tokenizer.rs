//! ICL Tokenizer — converts ICL text into token stream
//!
//! Handles: keywords, identifiers, string literals, integer/float literals,
//! ISO8601 timestamps, UUIDs, symbols (braces, colons, commas, brackets).
//! Comments (//) are discarded.
//!
//! Guarantees:
//! - Deterministic: same input always produces same token stream
//! - Complete error reporting: line:column for every error

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

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
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
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                tokens.push(SpannedToken {
                    token: Token::Eof,
                    span: self.current_span(),
                });
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    // ── Character helpers ──────────────────────────────────

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.position).copied();
        if let Some(c) = ch {
            self.position += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn current_span(&self) -> Span {
        Span {
            line: self.line,
            column: self.column,
            offset: self.position,
        }
    }

    // ── Whitespace & Comments ──────────────────────────────

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while let Some(ch) = self.peek() {
                if ch.is_ascii_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }

            // Skip line comments: //
            if self.peek() == Some('/') && self.peek_ahead(1) == Some('/') {
                while let Some(ch) = self.peek() {
                    if ch == '\n' {
                        break;
                    }
                    self.advance();
                }
                continue; // Loop back to skip more whitespace after comment
            }

            break;
        }
    }

    // ── Main dispatch ──────────────────────────────────────

    fn next_token(&mut self) -> crate::Result<SpannedToken> {
        let span = self.current_span();
        let ch = self.peek().unwrap();

        match ch {
            '{' => { self.advance(); Ok(SpannedToken { token: Token::LBrace, span }) }
            '}' => { self.advance(); Ok(SpannedToken { token: Token::RBrace, span }) }
            '[' => { self.advance(); Ok(SpannedToken { token: Token::LBracket, span }) }
            ']' => { self.advance(); Ok(SpannedToken { token: Token::RBracket, span }) }
            '<' => { self.advance(); Ok(SpannedToken { token: Token::LAngle, span }) }
            '>' => { self.advance(); Ok(SpannedToken { token: Token::RAngle, span }) }
            ':' => { self.advance(); Ok(SpannedToken { token: Token::Colon, span }) }
            ',' => { self.advance(); Ok(SpannedToken { token: Token::Comma, span }) }
            '=' => { self.advance(); Ok(SpannedToken { token: Token::Equals, span }) }
            '"' => self.read_string(span),
            c if c.is_ascii_digit() => self.read_number(span),
            c if c.is_ascii_alphabetic() || c == '_' => self.read_identifier_or_keyword(span),
            _ => Err(crate::Error::ParseError(
                format!("Unexpected character '{}' at {}", ch, span)
            )),
        }
    }

    // ── String literals ────────────────────────────────────

    fn read_string(&mut self, span: Span) -> crate::Result<SpannedToken> {
        self.advance(); // consume opening "
        let mut value = String::new();

        loop {
            match self.advance() {
                None => {
                    return Err(crate::Error::ParseError(
                        format!("Unterminated string starting at {}", span)
                    ));
                }
                Some('"') => break,
                Some('\\') => {
                    match self.advance() {
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some('\\') => value.push('\\'),
                        Some('"') => value.push('"'),
                        Some(c) => {
                            return Err(crate::Error::ParseError(
                                format!("Invalid escape sequence '\\{}' at {}", c, self.current_span())
                            ));
                        }
                        None => {
                            return Err(crate::Error::ParseError(
                                format!("Unterminated escape sequence at {}", self.current_span())
                            ));
                        }
                    }
                }
                Some(c) => value.push(c),
            }
        }

        Ok(SpannedToken {
            token: Token::StringLiteral(value),
            span,
        })
    }

    // ── Numbers & ISO8601 timestamps ───────────────────────

    fn read_number(&mut self, span: Span) -> crate::Result<SpannedToken> {
        let start = self.position;
        let mut has_dot = false;

        // Collect all digits
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        // Check for ISO8601: digits followed by '-' (like 2026-02-01T...)
        // Pattern: NNNN-NN-NNTNN:NN:NNZ
        if self.peek() == Some('-') && !has_dot {
            // Could be ISO8601 timestamp — collect the rest
            while let Some(ch) = self.peek() {
                if ch.is_ascii_alphanumeric() || ch == '-' || ch == ':' || ch == 'T' || ch == 'Z' || ch == '+' || ch == '.' {
                    self.advance();
                } else {
                    break;
                }
            }
            let text: String = self.input[start..self.position].iter().collect();
            // Validate basic ISO8601 shape
            if is_iso8601_like(&text) {
                return Ok(SpannedToken {
                    token: Token::StringLiteral(text),
                    span,
                });
            } else {
                return Err(crate::Error::ParseError(
                    format!("Invalid timestamp '{}' at {}", text, span)
                ));
            }
        }

        let text: String = self.input[start..self.position].iter().collect();

        if has_dot {
            let val: f64 = text.parse().map_err(|_| {
                crate::Error::ParseError(format!("Invalid float '{}' at {}", text, span))
            })?;
            Ok(SpannedToken {
                token: Token::FloatLiteral(val),
                span,
            })
        } else {
            let val: i64 = text.parse().map_err(|_| {
                crate::Error::ParseError(format!("Invalid integer '{}' at {}", text, span))
            })?;
            Ok(SpannedToken {
                token: Token::IntegerLiteral(val),
                span,
            })
        }
    }

    // ── Identifiers & Keywords ─────────────────────────────

    fn read_identifier_or_keyword(&mut self, span: Span) -> crate::Result<SpannedToken> {
        let start = self.position;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                self.advance();
            } else {
                break;
            }
        }

        let text: String = self.input[start..self.position].iter().collect();

        let token = match text.as_str() {
            // Section keywords
            "Contract" => Token::Contract,
            "Identity" => Token::Identity,
            "PurposeStatement" => Token::PurposeStatement,
            "DataSemantics" => Token::DataSemantics,
            "BehavioralSemantics" => Token::BehavioralSemantics,
            "ExecutionConstraints" => Token::ExecutionConstraints,
            "HumanMachineContract" => Token::HumanMachineContract,
            "Extensions" => Token::Extensions,

            // Type keywords
            "Integer" => Token::IntegerType,
            "Float" => Token::FloatType,
            "String" => Token::StringType,
            "Boolean" => Token::BooleanType,
            "ISO8601" => Token::Iso8601Type,
            "UUID" => Token::UuidType,
            "Array" => Token::ArrayType,
            "Map" => Token::MapType,
            "Object" => Token::ObjectType,
            "Enum" => Token::EnumType,

            // Boolean literals
            "true" => Token::BooleanLiteral(true),
            "false" => Token::BooleanLiteral(false),

            // Everything else is an identifier
            _ => Token::Identifier(text),
        };

        Ok(SpannedToken { token, span })
    }
}

/// Basic check for ISO8601-like timestamps (YYYY-MM-DDTHH:MM:SSZ)
fn is_iso8601_like(s: &str) -> bool {
    // Must contain T and end with Z or timezone offset
    // Minimal pattern: NNNN-NN-NNTNN:NN:NNZ
    if s.len() < 20 {
        return false;
    }
    s.contains('T') && (s.ends_with('Z') || s.contains('+'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        Tokenizer::new(input)
            .tokenize()
            .unwrap()
            .into_iter()
            .map(|st| st.token)
            .collect()
    }

    fn tokenize_err(input: &str) -> String {
        Tokenizer::new(input)
            .tokenize()
            .unwrap_err()
            .to_string()
    }

    // ── Keywords ───────────────────────────────────────

    #[test]
    fn test_tokenize_section_keywords() {
        let tokens = tokenize("Contract Identity PurposeStatement");
        assert_eq!(tokens, vec![
            Token::Contract,
            Token::Identity,
            Token::PurposeStatement,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_all_section_keywords() {
        let input = "Contract Identity PurposeStatement DataSemantics BehavioralSemantics ExecutionConstraints HumanMachineContract Extensions";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![
            Token::Contract,
            Token::Identity,
            Token::PurposeStatement,
            Token::DataSemantics,
            Token::BehavioralSemantics,
            Token::ExecutionConstraints,
            Token::HumanMachineContract,
            Token::Extensions,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_type_keywords() {
        let tokens = tokenize("Integer Float String Boolean ISO8601 UUID Array Map Object Enum");
        assert_eq!(tokens, vec![
            Token::IntegerType,
            Token::FloatType,
            Token::StringType,
            Token::BooleanType,
            Token::Iso8601Type,
            Token::UuidType,
            Token::ArrayType,
            Token::MapType,
            Token::ObjectType,
            Token::EnumType,
            Token::Eof,
        ]);
    }

    // ── String literals ────────────────────────────────

    #[test]
    fn test_tokenize_string_literal() {
        let tokens = tokenize(r#""hello world""#);
        assert_eq!(tokens, vec![
            Token::StringLiteral("hello world".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_string_escape_sequences() {
        let tokens = tokenize(r#""line\none\ttab\\slash\"quote""#);
        assert_eq!(tokens, vec![
            Token::StringLiteral("line\none\ttab\\slash\"quote".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_empty_string() {
        let tokens = tokenize(r#""""#);
        assert_eq!(tokens, vec![
            Token::StringLiteral(String::new()),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_unterminated_string() {
        let err = tokenize_err(r#""hello"#);
        assert!(err.contains("Unterminated string"));
    }

    // ── Numbers ────────────────────────────────────────

    #[test]
    fn test_tokenize_integer() {
        let tokens = tokenize("42 0 999999");
        assert_eq!(tokens, vec![
            Token::IntegerLiteral(42),
            Token::IntegerLiteral(0),
            Token::IntegerLiteral(999999),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_float() {
        let tokens = tokenize("3.14 0.0 1.0");
        assert_eq!(tokens, vec![
            Token::FloatLiteral(3.14),
            Token::FloatLiteral(0.0),
            Token::FloatLiteral(1.0),
            Token::Eof,
        ]);
    }

    // ── ISO8601 timestamps ─────────────────────────────

    #[test]
    fn test_tokenize_timestamp() {
        let tokens = tokenize("2026-02-01T00:00:00Z");
        assert_eq!(tokens, vec![
            Token::StringLiteral("2026-02-01T00:00:00Z".to_string()),
            Token::Eof,
        ]);
    }

    // ── Booleans ───────────────────────────────────────

    #[test]
    fn test_tokenize_booleans() {
        let tokens = tokenize("true false");
        assert_eq!(tokens, vec![
            Token::BooleanLiteral(true),
            Token::BooleanLiteral(false),
            Token::Eof,
        ]);
    }

    // ── Symbols ────────────────────────────────────────

    #[test]
    fn test_tokenize_symbols() {
        let tokens = tokenize("{ } [ ] < > : , =");
        assert_eq!(tokens, vec![
            Token::LBrace,
            Token::RBrace,
            Token::LBracket,
            Token::RBracket,
            Token::LAngle,
            Token::RAngle,
            Token::Colon,
            Token::Comma,
            Token::Equals,
            Token::Eof,
        ]);
    }

    // ── Comments ───────────────────────────────────────

    #[test]
    fn test_skip_line_comments() {
        let tokens = tokenize("Contract // this is a comment\nIdentity");
        assert_eq!(tokens, vec![
            Token::Contract,
            Token::Identity,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_skip_comment_at_start() {
        let tokens = tokenize("// comment\nContract");
        assert_eq!(tokens, vec![
            Token::Contract,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_skip_multiple_comments() {
        let tokens = tokenize("// first\n// second\nContract");
        assert_eq!(tokens, vec![
            Token::Contract,
            Token::Eof,
        ]);
    }

    // ── Identifiers ────────────────────────────────────

    #[test]
    fn test_tokenize_identifiers() {
        let tokens = tokenize("stable_id version count");
        assert_eq!(tokens, vec![
            Token::Identifier("stable_id".to_string()),
            Token::Identifier("version".to_string()),
            Token::Identifier("count".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_identifier_with_hyphens() {
        let tokens = tokenize("custom-system my-extension");
        assert_eq!(tokens, vec![
            Token::Identifier("custom-system".to_string()),
            Token::Identifier("my-extension".to_string()),
            Token::Eof,
        ]);
    }

    // ── Span tracking ──────────────────────────────────

    #[test]
    fn test_span_tracking() {
        let tokens = Tokenizer::new("Contract {\n  Identity\n}").tokenize().unwrap();
        assert_eq!(tokens[0].span, Span { line: 1, column: 1, offset: 0 });
        assert_eq!(tokens[0].token, Token::Contract);
        assert_eq!(tokens[1].span, Span { line: 1, column: 10, offset: 9 });
        assert_eq!(tokens[1].token, Token::LBrace);
        assert_eq!(tokens[2].span, Span { line: 2, column: 3, offset: 13 });
        assert_eq!(tokens[2].token, Token::Identity);
        assert_eq!(tokens[3].span, Span { line: 3, column: 1, offset: 22 });
        assert_eq!(tokens[3].token, Token::RBrace);
    }

    // ── Edge cases ─────────────────────────────────────

    #[test]
    fn test_empty_input() {
        let tokens = tokenize("");
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_only_whitespace() {
        let tokens = tokenize("   \n\n\t  ");
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_only_comments() {
        let tokens = tokenize("// nothing here\n// or here\n");
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_unexpected_character() {
        let err = tokenize_err("@");
        assert!(err.contains("Unexpected character"));
    }

    // ── Integration: minimal contract tokens ───────────

    #[test]
    fn test_tokenize_minimal_contract_fragment() {
        let input = r#"Contract {
  Identity {
    stable_id: "test-001",
    version: 1
  }
}"#;
        let tokens = tokenize(input);
        assert_eq!(tokens, vec![
            Token::Contract,
            Token::LBrace,
            Token::Identity,
            Token::LBrace,
            Token::Identifier("stable_id".to_string()),
            Token::Colon,
            Token::StringLiteral("test-001".to_string()),
            Token::Comma,
            Token::Identifier("version".to_string()),
            Token::Colon,
            Token::IntegerLiteral(1),
            Token::RBrace,
            Token::RBrace,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_type_expression() {
        let tokens = tokenize("Array<String>");
        assert_eq!(tokens, vec![
            Token::ArrayType,
            Token::LAngle,
            Token::StringType,
            Token::RAngle,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_tokenize_map_type() {
        let tokens = tokenize("Map<String, Integer>");
        assert_eq!(tokens, vec![
            Token::MapType,
            Token::LAngle,
            Token::StringType,
            Token::Comma,
            Token::IntegerType,
            Token::RAngle,
            Token::Eof,
        ]);
    }

    // ── Determinism proof ──────────────────────────────

    #[test]
    fn test_tokenize_determinism_100_iterations() {
        let input = r#"Contract {
  Identity {
    stable_id: "test",
    version: 1,
    created_timestamp: 2026-01-01T00:00:00Z,
    owner: "test",
    semantic_hash: "abc123"
  }
}"#;
        let first = Tokenizer::new(input).tokenize().unwrap();

        for i in 0..100 {
            let result = Tokenizer::new(input).tokenize().unwrap();
            assert_eq!(first, result, "Determinism failure at iteration {}", i);
        }
    }
}
