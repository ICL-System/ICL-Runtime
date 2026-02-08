//! ICL Parser — tokenizer, AST types, and recursive descent parser
//!
//! Converts ICL text into an Abstract Syntax Tree (AST).
//! Matches the BNF grammar defined in CORE-SPECIFICATION.md Section 1.
//!
//! # Pipeline
//!
//! `ICL text → Tokenizer → Token stream → Parser → ContractNode (AST)`
//!
//! # Guarantees
//!
//! - **Deterministic**: same input always produces identical AST
//! - **Pure**: no side effects, no I/O, no randomness
//! - **Complete errors**: line:column for every error

pub mod ast;
pub mod tokenizer;

use crate::{Error, Result};
use ast::*;
use tokenizer::{Span, SpannedToken, Token, Tokenizer};

// ── Public API ─────────────────────────────────────────────

/// Parse ICL text into an AST ContractNode
///
/// Returns the raw parse tree with source positions (spans).
/// For a semantic `Contract`, use [`parse_contract`] instead.
///
/// # Guarantees
/// - Deterministic: same input always produces same AST
/// - Error messages include line:column
///
/// # Errors
/// Returns `ParseError` with line:column for syntax violations.
pub fn parse(input: &str) -> Result<ContractNode> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_contract_definition()
}

/// Parse ICL text into a semantic Contract (parse + lower)
///
/// Combines parsing (text → AST) with lowering (AST → semantic Contract).
///
/// # Errors
/// Returns `ParseError` for syntax errors or `ValidationError` for
/// semantic issues (e.g., confidence_level out of range).
pub fn parse_contract(input: &str) -> Result<crate::Contract> {
    let node = parse(input)?;
    lower_contract(&node)
}

// ── Parser ─────────────────────────────────────────────────

struct Parser {
    tokens: Vec<SpannedToken>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<SpannedToken>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    // ── Token helpers ──────────────────────────────────

    fn current_span(&self) -> Span {
        if self.position < self.tokens.len() {
            self.tokens[self.position].span.clone()
        } else {
            Span {
                line: 0,
                column: 0,
                offset: 0,
            }
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position].token
    }

    fn advance(&mut self) -> SpannedToken {
        let token = self.tokens[self.position].clone();
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
        token
    }

    /// Expect a specific token (exact match for keywords/symbols)
    fn expect(&mut self, expected: Token) -> Result<SpannedToken> {
        let current = self.tokens[self.position].clone();
        if current.token == expected {
            self.advance();
            Ok(current)
        } else {
            Err(Error::ParseError(format!(
                "Expected {:?}, found {:?} at {}",
                expected, current.token, current.span
            )))
        }
    }

    /// Expect a string literal and return its value with span
    fn expect_string_literal(&mut self) -> Result<SpannedValue<String>> {
        let st = self.advance();
        match st.token {
            Token::StringLiteral(s) => Ok(SpannedValue::new(s, st.span)),
            _ => Err(Error::ParseError(format!(
                "Expected string literal, found {:?} at {}",
                st.token, st.span
            ))),
        }
    }

    /// Expect an integer literal and return its value with span
    fn expect_integer_literal(&mut self) -> Result<SpannedValue<i64>> {
        let st = self.advance();
        match st.token {
            Token::IntegerLiteral(n) => Ok(SpannedValue::new(n, st.span)),
            _ => Err(Error::ParseError(format!(
                "Expected integer literal, found {:?} at {}",
                st.token, st.span
            ))),
        }
    }

    /// Expect a float literal and return its value with span
    fn expect_float_literal(&mut self) -> Result<SpannedValue<f64>> {
        let st = self.advance();
        match st.token {
            Token::FloatLiteral(f) => Ok(SpannedValue::new(f, st.span)),
            _ => Err(Error::ParseError(format!(
                "Expected float literal, found {:?} at {}",
                st.token, st.span
            ))),
        }
    }

    /// Expect a named field: `identifier ":"`
    fn expect_field(&mut self, name: &str) -> Result<Span> {
        let st = self.advance();
        match &st.token {
            Token::Identifier(id) if id == name => {}
            _ => {
                return Err(Error::ParseError(format!(
                    "Expected field '{}', found {:?} at {}",
                    name, st.token, st.span
                )));
            }
        }
        self.expect(Token::Colon)?;
        Ok(st.span)
    }

    /// Consume a comma if present (for optional trailing commas)
    fn optional_comma(&mut self) {
        if matches!(self.peek(), Token::Comma) {
            self.advance();
        }
    }

    /// Peek at the current token and return identifier name without advancing
    fn peek_identifier_name(&self) -> Result<String> {
        match &self.tokens[self.position].token {
            Token::Identifier(name) => Ok(name.clone()),
            other => Err(Error::ParseError(format!(
                "Expected field name identifier, found {:?} at {}",
                other, self.tokens[self.position].span
            ))),
        }
    }

    // ── Top-level parsing ──────────────────────────────

    /// Parse: `Contract { ... } [Extensions { ... }]`
    fn parse_contract_definition(&mut self) -> Result<ContractNode> {
        let span = self.current_span();
        self.expect(Token::Contract)?;
        self.expect(Token::LBrace)?;

        let identity = self.parse_identity()?;
        let purpose_statement = self.parse_purpose_statement()?;
        let data_semantics = self.parse_data_semantics()?;
        let behavioral_semantics = self.parse_behavioral_semantics()?;
        let execution_constraints = self.parse_execution_constraints()?;
        let human_machine_contract = self.parse_human_machine_contract()?;

        self.expect(Token::RBrace)?;

        // Optional Extensions block (outside Contract per BNF §5)
        let extensions = if matches!(self.peek(), Token::Extensions) {
            Some(self.parse_extensions()?)
        } else {
            None
        };

        Ok(ContractNode {
            identity,
            purpose_statement,
            data_semantics,
            behavioral_semantics,
            execution_constraints,
            human_machine_contract,
            extensions,
            span,
        })
    }

    // ── Identity (§1.2) ───────────────────────────────

    fn parse_identity(&mut self) -> Result<IdentityNode> {
        let span = self.current_span();
        self.expect(Token::Identity)?;
        self.expect(Token::LBrace)?;

        let mut stable_id: Option<SpannedValue<String>> = None;
        let mut version: Option<SpannedValue<i64>> = None;
        let mut created_timestamp: Option<SpannedValue<String>> = None;
        let mut owner: Option<SpannedValue<String>> = None;
        let mut semantic_hash: Option<SpannedValue<String>> = None;

        while !matches!(self.peek(), Token::RBrace) {
            let field_name = self.peek_identifier_name()?;
            match field_name.as_str() {
                "stable_id" => {
                    self.expect_field("stable_id")?;
                    stable_id = Some(self.expect_string_literal()?);
                }
                "version" => {
                    self.expect_field("version")?;
                    version = Some(self.expect_integer_literal()?);
                }
                "created_timestamp" => {
                    self.expect_field("created_timestamp")?;
                    created_timestamp = Some(self.expect_string_literal()?);
                }
                "owner" => {
                    self.expect_field("owner")?;
                    owner = Some(self.expect_string_literal()?);
                }
                "semantic_hash" => {
                    self.expect_field("semantic_hash")?;
                    semantic_hash = Some(self.expect_string_literal()?);
                }
                other => {
                    return Err(Error::ParseError(format!(
                        "Unknown field '{}' in Identity at {}",
                        other,
                        self.current_span()
                    )));
                }
            }
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(IdentityNode {
            stable_id: stable_id.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'stable_id' in Identity at {}",
                    span
                ))
            })?,
            version: version.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'version' in Identity at {}",
                    span
                ))
            })?,
            created_timestamp: created_timestamp.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'created_timestamp' in Identity at {}",
                    span
                ))
            })?,
            owner: owner.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'owner' in Identity at {}",
                    span
                ))
            })?,
            semantic_hash: semantic_hash.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'semantic_hash' in Identity at {}",
                    span
                ))
            })?,
            span,
        })
    }

    // ── PurposeStatement (§1.3) ───────────────────────

    fn parse_purpose_statement(&mut self) -> Result<PurposeStatementNode> {
        let span = self.current_span();
        self.expect(Token::PurposeStatement)?;
        self.expect(Token::LBrace)?;

        let mut narrative: Option<SpannedValue<String>> = None;
        let mut intent_source: Option<SpannedValue<String>> = None;
        let mut confidence_level: Option<SpannedValue<f64>> = None;

        while !matches!(self.peek(), Token::RBrace) {
            let field_name = self.peek_identifier_name()?;
            match field_name.as_str() {
                "narrative" => {
                    self.expect_field("narrative")?;
                    narrative = Some(self.expect_string_literal()?);
                }
                "intent_source" => {
                    self.expect_field("intent_source")?;
                    intent_source = Some(self.expect_string_literal()?);
                }
                "confidence_level" => {
                    self.expect_field("confidence_level")?;
                    let cl = self.expect_float_literal()?;
                    if cl.value < 0.0 || cl.value > 1.0 {
                        return Err(Error::ValidationError(format!(
                            "confidence_level must be in [0.0, 1.0], found {} at {}",
                            cl.value, cl.span
                        )));
                    }
                    confidence_level = Some(cl);
                }
                other => {
                    return Err(Error::ParseError(format!(
                        "Unknown field '{}' in PurposeStatement at {}",
                        other,
                        self.current_span()
                    )));
                }
            }
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(PurposeStatementNode {
            narrative: narrative.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'narrative' in PurposeStatement at {}",
                    span
                ))
            })?,
            intent_source: intent_source.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'intent_source' in PurposeStatement at {}",
                    span
                ))
            })?,
            confidence_level: confidence_level.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'confidence_level' in PurposeStatement at {}",
                    span
                ))
            })?,
            span,
        })
    }

    // ── DataSemantics (§1.4) ──────────────────────────

    fn parse_data_semantics(&mut self) -> Result<DataSemanticsNode> {
        let span = self.current_span();
        self.expect(Token::DataSemantics)?;
        self.expect(Token::LBrace)?;

        let mut state: Option<Vec<StateFieldNode>> = None;
        let mut invariants: Option<Vec<SpannedValue<String>>> = None;

        while !matches!(self.peek(), Token::RBrace) {
            let field_name = self.peek_identifier_name()?;
            match field_name.as_str() {
                "state" => {
                    self.expect_field("state")?;
                    self.expect(Token::LBrace)?;
                    state = Some(self.parse_state_fields()?);
                    self.expect(Token::RBrace)?;
                }
                "invariants" => {
                    self.expect_field("invariants")?;
                    invariants = Some(self.parse_string_list()?);
                }
                other => {
                    return Err(Error::ParseError(format!(
                        "Unknown field '{}' in DataSemantics at {}",
                        other,
                        self.current_span()
                    )));
                }
            }
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(DataSemanticsNode {
            state: state.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'state' in DataSemantics at {}",
                    span
                ))
            })?,
            invariants: invariants.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'invariants' in DataSemantics at {}",
                    span
                ))
            })?,
            span,
        })
    }

    /// Parse state field list: `field1: Type, field2: Type = default, ...`
    fn parse_state_fields(&mut self) -> Result<Vec<StateFieldNode>> {
        let mut fields = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            fields.push(self.parse_state_field()?);
            self.optional_comma();
        }
        Ok(fields)
    }

    /// Parse a single state field: `name: TypeExpression [= default]`
    fn parse_state_field(&mut self) -> Result<StateFieldNode> {
        let span = self.current_span();

        let name_st = self.advance();
        let name = match name_st.token {
            Token::Identifier(s) => SpannedValue::new(s, name_st.span),
            _ => {
                return Err(Error::ParseError(format!(
                    "Expected field name, found {:?} at {}",
                    name_st.token, name_st.span
                )));
            }
        };

        self.expect(Token::Colon)?;
        let type_expr = self.parse_type_expression()?;

        let default_value = if matches!(self.peek(), Token::Equals) {
            self.advance(); // consume =
            Some(self.parse_literal_value()?)
        } else {
            None
        };

        Ok(StateFieldNode {
            name,
            type_expr,
            default_value,
            span,
        })
    }

    // ── Type Expressions ──────────────────────────────

    fn parse_type_expression(&mut self) -> Result<TypeExpression> {
        let span = self.current_span();
        match self.peek().clone() {
            Token::IntegerType => {
                self.advance();
                Ok(TypeExpression::Primitive(PrimitiveType::Integer, span))
            }
            Token::FloatType => {
                self.advance();
                Ok(TypeExpression::Primitive(PrimitiveType::Float, span))
            }
            Token::StringType => {
                self.advance();
                Ok(TypeExpression::Primitive(PrimitiveType::String, span))
            }
            Token::BooleanType => {
                self.advance();
                Ok(TypeExpression::Primitive(PrimitiveType::Boolean, span))
            }
            Token::Iso8601Type => {
                self.advance();
                Ok(TypeExpression::Primitive(PrimitiveType::Iso8601, span))
            }
            Token::UuidType => {
                self.advance();
                Ok(TypeExpression::Primitive(PrimitiveType::Uuid, span))
            }
            Token::ArrayType => self.parse_array_type(span),
            Token::MapType => self.parse_map_type(span),
            Token::ObjectType => self.parse_object_type(span),
            Token::EnumType => self.parse_enum_type(span),
            _ => Err(Error::ParseError(format!(
                "Expected type expression, found {:?} at {}",
                self.peek(),
                span
            ))),
        }
    }

    fn parse_array_type(&mut self, span: Span) -> Result<TypeExpression> {
        self.advance(); // consume Array
        self.expect(Token::LAngle)?;
        let inner = self.parse_type_expression()?;
        self.expect(Token::RAngle)?;
        Ok(TypeExpression::Array(Box::new(inner), span))
    }

    fn parse_map_type(&mut self, span: Span) -> Result<TypeExpression> {
        self.advance(); // consume Map
        self.expect(Token::LAngle)?;
        let key = self.parse_type_expression()?;
        self.expect(Token::Comma)?;
        let value = self.parse_type_expression()?;
        self.expect(Token::RAngle)?;
        Ok(TypeExpression::Map(Box::new(key), Box::new(value), span))
    }

    fn parse_object_type(&mut self, span: Span) -> Result<TypeExpression> {
        self.advance(); // consume Object
        self.expect(Token::LBrace)?;
        let fields = self.parse_state_fields()?;
        self.expect(Token::RBrace)?;
        Ok(TypeExpression::Object(fields, span))
    }

    fn parse_enum_type(&mut self, span: Span) -> Result<TypeExpression> {
        self.advance(); // consume Enum
        self.expect(Token::LBracket)?;

        let mut variants = Vec::new();
        if !matches!(self.peek(), Token::RBracket) {
            variants.push(self.expect_string_literal()?);
            while matches!(self.peek(), Token::Comma) {
                self.advance(); // consume comma
                if matches!(self.peek(), Token::RBracket) {
                    break; // trailing comma
                }
                variants.push(self.expect_string_literal()?);
            }
        }

        self.expect(Token::RBracket)?;
        Ok(TypeExpression::Enum(variants, span))
    }

    // ── Literal Values ────────────────────────────────

    fn parse_literal_value(&mut self) -> Result<LiteralValue> {
        let span = self.current_span();
        match self.peek().clone() {
            Token::StringLiteral(_) => {
                let st = self.advance();
                if let Token::StringLiteral(s) = st.token {
                    Ok(LiteralValue::String(s, st.span))
                } else {
                    unreachable!()
                }
            }
            Token::IntegerLiteral(_) => {
                let st = self.advance();
                if let Token::IntegerLiteral(n) = st.token {
                    Ok(LiteralValue::Integer(n, st.span))
                } else {
                    unreachable!()
                }
            }
            Token::FloatLiteral(_) => {
                let st = self.advance();
                if let Token::FloatLiteral(f) = st.token {
                    Ok(LiteralValue::Float(f, st.span))
                } else {
                    unreachable!()
                }
            }
            Token::BooleanLiteral(_) => {
                let st = self.advance();
                if let Token::BooleanLiteral(b) = st.token {
                    Ok(LiteralValue::Boolean(b, st.span))
                } else {
                    unreachable!()
                }
            }
            Token::LBracket => {
                self.advance(); // consume [
                let mut items = Vec::new();
                if !matches!(self.peek(), Token::RBracket) {
                    items.push(self.parse_literal_value()?);
                    while matches!(self.peek(), Token::Comma) {
                        self.advance(); // consume comma
                        if matches!(self.peek(), Token::RBracket) {
                            break; // trailing comma
                        }
                        items.push(self.parse_literal_value()?);
                    }
                }
                self.expect(Token::RBracket)?;
                Ok(LiteralValue::Array(items, span))
            }
            _ => Err(Error::ParseError(format!(
                "Expected literal value, found {:?} at {}",
                self.peek(),
                span
            ))),
        }
    }

    // ── BehavioralSemantics (§1.5) ────────────────────

    fn parse_behavioral_semantics(&mut self) -> Result<BehavioralSemanticsNode> {
        let span = self.current_span();
        self.expect(Token::BehavioralSemantics)?;
        self.expect(Token::LBrace)?;

        self.expect_field("operations")?;
        self.expect(Token::LBracket)?;

        let mut operations = Vec::new();
        while !matches!(self.peek(), Token::RBracket) {
            operations.push(self.parse_operation()?);
            self.optional_comma();
        }

        self.expect(Token::RBracket)?;
        self.optional_comma();

        self.expect(Token::RBrace)?;

        Ok(BehavioralSemanticsNode { operations, span })
    }

    fn parse_operation(&mut self) -> Result<OperationNode> {
        let span = self.current_span();
        self.expect(Token::LBrace)?;

        let mut name: Option<SpannedValue<String>> = None;
        let mut precondition: Option<SpannedValue<String>> = None;
        let mut parameters: Option<Vec<StateFieldNode>> = None;
        let mut postcondition: Option<SpannedValue<String>> = None;
        let mut side_effects: Option<Vec<SpannedValue<String>>> = None;
        let mut idempotence: Option<SpannedValue<String>> = None;

        while !matches!(self.peek(), Token::RBrace) {
            let field_name = self.peek_identifier_name()?;
            match field_name.as_str() {
                "name" => {
                    self.expect_field("name")?;
                    name = Some(self.expect_string_literal()?);
                }
                "precondition" => {
                    self.expect_field("precondition")?;
                    precondition = Some(self.expect_string_literal()?);
                }
                "parameters" => {
                    self.expect_field("parameters")?;
                    self.expect(Token::LBrace)?;
                    parameters = Some(self.parse_state_fields()?);
                    self.expect(Token::RBrace)?;
                }
                "postcondition" => {
                    self.expect_field("postcondition")?;
                    postcondition = Some(self.expect_string_literal()?);
                }
                "side_effects" => {
                    self.expect_field("side_effects")?;
                    side_effects = Some(self.parse_string_list()?);
                }
                "idempotence" => {
                    self.expect_field("idempotence")?;
                    idempotence = Some(self.expect_string_literal()?);
                }
                other => {
                    return Err(Error::ParseError(format!(
                        "Unknown field '{}' in operation at {}",
                        other,
                        self.current_span()
                    )));
                }
            }
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(OperationNode {
            name: name.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'name' in operation at {}",
                    span
                ))
            })?,
            precondition: precondition.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'precondition' in operation at {}",
                    span
                ))
            })?,
            parameters: parameters.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'parameters' in operation at {}",
                    span
                ))
            })?,
            postcondition: postcondition.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'postcondition' in operation at {}",
                    span
                ))
            })?,
            side_effects: side_effects.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'side_effects' in operation at {}",
                    span
                ))
            })?,
            idempotence: idempotence.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'idempotence' in operation at {}",
                    span
                ))
            })?,
            span,
        })
    }

    // ── ExecutionConstraints (§1.6) ───────────────────

    fn parse_execution_constraints(&mut self) -> Result<ExecutionConstraintsNode> {
        let span = self.current_span();
        self.expect(Token::ExecutionConstraints)?;
        self.expect(Token::LBrace)?;

        let mut trigger_types: Option<Vec<SpannedValue<String>>> = None;
        let mut resource_limits: Option<ResourceLimitsNode> = None;
        let mut external_permissions: Option<Vec<SpannedValue<String>>> = None;
        let mut sandbox_mode: Option<SpannedValue<String>> = None;

        while !matches!(self.peek(), Token::RBrace) {
            let field_name = self.peek_identifier_name()?;
            match field_name.as_str() {
                "trigger_types" => {
                    self.expect_field("trigger_types")?;
                    trigger_types = Some(self.parse_string_list()?);
                }
                "resource_limits" => {
                    self.expect_field("resource_limits")?;
                    resource_limits = Some(self.parse_resource_limits()?);
                }
                "external_permissions" => {
                    self.expect_field("external_permissions")?;
                    external_permissions = Some(self.parse_string_list()?);
                }
                "sandbox_mode" => {
                    self.expect_field("sandbox_mode")?;
                    sandbox_mode = Some(self.expect_string_literal()?);
                }
                other => {
                    return Err(Error::ParseError(format!(
                        "Unknown field '{}' in ExecutionConstraints at {}",
                        other,
                        self.current_span()
                    )));
                }
            }
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(ExecutionConstraintsNode {
            trigger_types: trigger_types.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'trigger_types' in ExecutionConstraints at {}",
                    span
                ))
            })?,
            resource_limits: resource_limits.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'resource_limits' in ExecutionConstraints at {}",
                    span
                ))
            })?,
            external_permissions: external_permissions.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'external_permissions' in ExecutionConstraints at {}",
                    span
                ))
            })?,
            sandbox_mode: sandbox_mode.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'sandbox_mode' in ExecutionConstraints at {}",
                    span
                ))
            })?,
            span,
        })
    }

    fn parse_resource_limits(&mut self) -> Result<ResourceLimitsNode> {
        let span = self.current_span();
        self.expect(Token::LBrace)?;

        let mut max_memory_bytes: Option<SpannedValue<i64>> = None;
        let mut computation_timeout_ms: Option<SpannedValue<i64>> = None;
        let mut max_state_size_bytes: Option<SpannedValue<i64>> = None;

        while !matches!(self.peek(), Token::RBrace) {
            let field_name = self.peek_identifier_name()?;
            match field_name.as_str() {
                "max_memory_bytes" => {
                    self.expect_field("max_memory_bytes")?;
                    max_memory_bytes = Some(self.expect_integer_literal()?);
                }
                "computation_timeout_ms" => {
                    self.expect_field("computation_timeout_ms")?;
                    computation_timeout_ms = Some(self.expect_integer_literal()?);
                }
                "max_state_size_bytes" => {
                    self.expect_field("max_state_size_bytes")?;
                    max_state_size_bytes = Some(self.expect_integer_literal()?);
                }
                other => {
                    return Err(Error::ParseError(format!(
                        "Unknown field '{}' in resource_limits at {}",
                        other,
                        self.current_span()
                    )));
                }
            }
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(ResourceLimitsNode {
            max_memory_bytes: max_memory_bytes.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'max_memory_bytes' in resource_limits at {}",
                    span
                ))
            })?,
            computation_timeout_ms: computation_timeout_ms.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'computation_timeout_ms' in resource_limits at {}",
                    span
                ))
            })?,
            max_state_size_bytes: max_state_size_bytes.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'max_state_size_bytes' in resource_limits at {}",
                    span
                ))
            })?,
            span,
        })
    }

    // ── HumanMachineContract (§1.7) ───────────────────

    fn parse_human_machine_contract(&mut self) -> Result<HumanMachineContractNode> {
        let span = self.current_span();
        self.expect(Token::HumanMachineContract)?;
        self.expect(Token::LBrace)?;

        let mut system_commitments: Option<Vec<SpannedValue<String>>> = None;
        let mut system_refusals: Option<Vec<SpannedValue<String>>> = None;
        let mut user_obligations: Option<Vec<SpannedValue<String>>> = None;

        while !matches!(self.peek(), Token::RBrace) {
            let field_name = self.peek_identifier_name()?;
            match field_name.as_str() {
                "system_commitments" => {
                    self.expect_field("system_commitments")?;
                    system_commitments = Some(self.parse_string_list()?);
                }
                "system_refusals" => {
                    self.expect_field("system_refusals")?;
                    system_refusals = Some(self.parse_string_list()?);
                }
                "user_obligations" => {
                    self.expect_field("user_obligations")?;
                    user_obligations = Some(self.parse_string_list()?);
                }
                other => {
                    return Err(Error::ParseError(format!(
                        "Unknown field '{}' in HumanMachineContract at {}",
                        other,
                        self.current_span()
                    )));
                }
            }
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(HumanMachineContractNode {
            system_commitments: system_commitments.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'system_commitments' in HumanMachineContract at {}",
                    span
                ))
            })?,
            system_refusals: system_refusals.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'system_refusals' in HumanMachineContract at {}",
                    span
                ))
            })?,
            user_obligations: user_obligations.ok_or_else(|| {
                Error::ParseError(format!(
                    "Missing required field 'user_obligations' in HumanMachineContract at {}",
                    span
                ))
            })?,
            span,
        })
    }

    // ── Extensions (§5) ───────────────────────────────

    fn parse_extensions(&mut self) -> Result<ExtensionsNode> {
        let span = self.current_span();
        self.expect(Token::Extensions)?;
        self.expect(Token::LBrace)?;

        let mut systems = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            systems.push(self.parse_system_extension()?);
        }

        self.expect(Token::RBrace)?;

        Ok(ExtensionsNode { systems, span })
    }

    fn parse_system_extension(&mut self) -> Result<SystemExtensionNode> {
        let span = self.current_span();

        let name_st = self.advance();
        let name = match name_st.token {
            Token::Identifier(s) => SpannedValue::new(s, name_st.span),
            _ => {
                return Err(Error::ParseError(format!(
                    "Expected system extension name, found {:?} at {}",
                    name_st.token, name_st.span
                )));
            }
        };

        self.expect(Token::LBrace)?;

        let mut fields = Vec::new();
        while !matches!(self.peek(), Token::RBrace) {
            fields.push(self.parse_custom_field()?);
            self.optional_comma();
        }

        self.expect(Token::RBrace)?;

        Ok(SystemExtensionNode { name, fields, span })
    }

    fn parse_custom_field(&mut self) -> Result<CustomFieldNode> {
        let span = self.current_span();

        let name_st = self.advance();
        let name = match name_st.token {
            Token::Identifier(s) => SpannedValue::new(s, name_st.span),
            _ => {
                return Err(Error::ParseError(format!(
                    "Expected field name, found {:?} at {}",
                    name_st.token, name_st.span
                )));
            }
        };

        self.expect(Token::Colon)?;
        let value = self.parse_literal_value()?;

        Ok(CustomFieldNode { name, value, span })
    }

    // ── Helpers ────────────────────────────────────────

    /// Parse: `[ "str1", "str2", ... ]`
    fn parse_string_list(&mut self) -> Result<Vec<SpannedValue<String>>> {
        self.expect(Token::LBracket)?;

        let mut items = Vec::new();
        if !matches!(self.peek(), Token::RBracket) {
            items.push(self.expect_string_literal()?);
            while matches!(self.peek(), Token::Comma) {
                self.advance(); // consume comma
                if matches!(self.peek(), Token::RBracket) {
                    break; // trailing comma
                }
                items.push(self.expect_string_literal()?);
            }
        }

        self.expect(Token::RBracket)?;
        Ok(items)
    }
}

// ── Lowering: AST → semantic Contract ──────────────────────

/// Convert a parsed AST into a runtime Contract struct.
/// This is the bridge between the parser output and the executor input.
pub fn lower_contract(node: &ContractNode) -> Result<crate::Contract> {
    Ok(crate::Contract {
        identity: crate::Identity {
            stable_id: node.identity.stable_id.value.clone(),
            version: node.identity.version.value as u32,
            created_timestamp: node.identity.created_timestamp.value.clone(),
            owner: node.identity.owner.value.clone(),
            semantic_hash: node.identity.semantic_hash.value.clone(),
        },
        purpose_statement: crate::PurposeStatement {
            narrative: node.purpose_statement.narrative.value.clone(),
            intent_source: node.purpose_statement.intent_source.value.clone(),
            confidence_level: node.purpose_statement.confidence_level.value,
        },
        data_semantics: lower_data_semantics(&node.data_semantics),
        behavioral_semantics: lower_behavioral_semantics(&node.behavioral_semantics),
        execution_constraints: crate::ExecutionConstraints {
            trigger_types: node
                .execution_constraints
                .trigger_types
                .iter()
                .map(|s| s.value.clone())
                .collect(),
            resource_limits: crate::ResourceLimits {
                max_memory_bytes: node
                    .execution_constraints
                    .resource_limits
                    .max_memory_bytes
                    .value as u64,
                computation_timeout_ms: node
                    .execution_constraints
                    .resource_limits
                    .computation_timeout_ms
                    .value as u64,
                max_state_size_bytes: node
                    .execution_constraints
                    .resource_limits
                    .max_state_size_bytes
                    .value as u64,
            },
            external_permissions: node
                .execution_constraints
                .external_permissions
                .iter()
                .map(|s| s.value.clone())
                .collect(),
            sandbox_mode: node.execution_constraints.sandbox_mode.value.clone(),
        },
        human_machine_contract: crate::HumanMachineContract {
            system_commitments: node
                .human_machine_contract
                .system_commitments
                .iter()
                .map(|s| s.value.clone())
                .collect(),
            system_refusals: node
                .human_machine_contract
                .system_refusals
                .iter()
                .map(|s| s.value.clone())
                .collect(),
            user_obligations: node
                .human_machine_contract
                .user_obligations
                .iter()
                .map(|s| s.value.clone())
                .collect(),
        },
    })
}

fn lower_data_semantics(node: &DataSemanticsNode) -> crate::DataSemantics {
    let mut state = serde_json::Map::new();
    for field in &node.state {
        let type_str = field.type_expr.to_string();
        let value = if let Some(ref default) = field.default_value {
            // Store as {"type": "...", "default": value} to preserve defaults
            let default_json = lower_literal(default);
            serde_json::json!({
                "type": type_str,
                "default": default_json
            })
        } else {
            serde_json::Value::String(type_str)
        };
        state.insert(field.name.value.clone(), value);
    }
    crate::DataSemantics {
        state: serde_json::Value::Object(state),
        invariants: node.invariants.iter().map(|s| s.value.clone()).collect(),
    }
}

fn lower_literal(lit: &ast::LiteralValue) -> serde_json::Value {
    match lit {
        ast::LiteralValue::String(s, _) => serde_json::Value::String(s.clone()),
        ast::LiteralValue::Integer(i, _) => serde_json::json!(*i),
        ast::LiteralValue::Float(f, _) => serde_json::json!(*f),
        ast::LiteralValue::Boolean(b, _) => serde_json::Value::Bool(*b),
        ast::LiteralValue::Array(arr, _) => {
            serde_json::Value::Array(arr.iter().map(lower_literal).collect())
        }
    }
}

fn lower_behavioral_semantics(node: &BehavioralSemanticsNode) -> crate::BehavioralSemantics {
    let operations = node
        .operations
        .iter()
        .map(|op| {
            let mut params = serde_json::Map::new();
            for p in &op.parameters {
                params.insert(
                    p.name.value.clone(),
                    serde_json::Value::String(p.type_expr.to_string()),
                );
            }

            crate::Operation {
                name: op.name.value.clone(),
                precondition: op.precondition.value.clone(),
                parameters: serde_json::Value::Object(params),
                postcondition: op.postcondition.value.clone(),
                side_effects: op.side_effects.iter().map(|s| s.value.clone()).collect(),
                idempotence: op.idempotence.value.clone(),
            }
        })
        .collect();

    crate::BehavioralSemantics { operations }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    // ── Helper ─────────────────────────────────────────

    fn parse_valid(input: &str) -> ContractNode {
        parse(input).unwrap_or_else(|e| panic!("Expected successful parse, got: {}", e))
    }

    fn parse_err(input: &str) -> String {
        parse(input).unwrap_err().to_string()
    }

    // ── Minimal contract ───────────────────────────────

    const MINIMAL_CONTRACT: &str = r#"Contract {
  Identity {
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }

  PurposeStatement {
    narrative: "Minimal test contract",
    intent_source: "test",
    confidence_level: 1.0
  }

  DataSemantics {
    state: {
      value: String
    },
    invariants: []
  }

  BehavioralSemantics {
    operations: []
  }

  ExecutionConstraints {
    trigger_types: ["manual"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }

  HumanMachineContract {
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}"#;

    #[test]
    fn test_parse_minimal_contract() {
        let ast = parse_valid(MINIMAL_CONTRACT);
        assert_eq!(ast.identity.stable_id.value, "ic-test-001");
        assert_eq!(ast.identity.version.value, 1);
        assert_eq!(ast.identity.owner.value, "test");
        assert_eq!(ast.purpose_statement.confidence_level.value, 1.0);
        assert_eq!(ast.data_semantics.state.len(), 1);
        assert_eq!(ast.data_semantics.state[0].name.value, "value");
        assert_eq!(ast.behavioral_semantics.operations.len(), 0);
        assert!(ast.extensions.is_none());
    }

    #[test]
    fn test_parse_contract_lowers_correctly() {
        let contract = parse_contract(MINIMAL_CONTRACT).unwrap();
        assert_eq!(contract.identity.stable_id, "ic-test-001");
        assert_eq!(contract.identity.version, 1);
        assert_eq!(contract.purpose_statement.confidence_level, 1.0);
        assert_eq!(
            contract.execution_constraints.sandbox_mode,
            "full_isolation"
        );
    }

    // ── Type expressions ───────────────────────────────

    #[test]
    fn test_parse_all_primitive_types() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-types-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "1111111111111111"
  }
  PurposeStatement {
    narrative: "Primitive types test",
    intent_source: "test",
    confidence_level: 0.95
  }
  DataSemantics {
    state: {
      count: Integer = 0,
      ratio: Float = 1.0,
      label: String,
      active: Boolean,
      created_at: ISO8601,
      user_id: UUID
    },
    invariants: ["count >= 0"]
  }
  BehavioralSemantics {
    operations: []
  }
  ExecutionConstraints {
    trigger_types: ["manual"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }
  HumanMachineContract {
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}"#;
        let ast = parse_valid(input);
        assert_eq!(ast.data_semantics.state.len(), 6);

        let state = &ast.data_semantics.state;
        assert_eq!(state[0].name.value, "count");
        assert!(matches!(
            state[0].type_expr,
            TypeExpression::Primitive(PrimitiveType::Integer, _)
        ));
        assert!(state[0].default_value.is_some());

        assert_eq!(state[3].name.value, "active");
        assert!(matches!(
            state[3].type_expr,
            TypeExpression::Primitive(PrimitiveType::Boolean, _)
        ));

        assert_eq!(state[4].name.value, "created_at");
        assert!(matches!(
            state[4].type_expr,
            TypeExpression::Primitive(PrimitiveType::Iso8601, _)
        ));

        assert_eq!(state[5].name.value, "user_id");
        assert!(matches!(
            state[5].type_expr,
            TypeExpression::Primitive(PrimitiveType::Uuid, _)
        ));
    }

    #[test]
    fn test_parse_composite_types() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-composite-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "2222222222222222"
  }
  PurposeStatement {
    narrative: "Composite types",
    intent_source: "test",
    confidence_level: 1.0
  }
  DataSemantics {
    state: {
      status: Enum["pending", "active", "completed"],
      metadata: Object {
        key: String,
        value: String
      },
      tags: Array<String>,
      scores: Map<String, Integer>
    },
    invariants: ["status is valid enum value"]
  }
  BehavioralSemantics {
    operations: []
  }
  ExecutionConstraints {
    trigger_types: ["manual"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }
  HumanMachineContract {
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}"#;
        let ast = parse_valid(input);
        let state = &ast.data_semantics.state;
        assert_eq!(state.len(), 4);

        // Enum
        if let TypeExpression::Enum(variants, _) = &state[0].type_expr {
            assert_eq!(variants.len(), 3);
            assert_eq!(variants[0].value, "pending");
            assert_eq!(variants[2].value, "completed");
        } else {
            panic!("Expected Enum type");
        }

        // Object
        if let TypeExpression::Object(fields, _) = &state[1].type_expr {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name.value, "key");
        } else {
            panic!("Expected Object type");
        }

        // Array<String>
        assert!(matches!(&state[2].type_expr, TypeExpression::Array(_, _)));

        // Map<String, Integer>
        assert!(matches!(&state[3].type_expr, TypeExpression::Map(_, _, _)));
    }

    // ── Operations ─────────────────────────────────────

    #[test]
    fn test_parse_multiple_operations() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-ops-001",
    version: 2,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "4444444444444444"
  }
  PurposeStatement {
    narrative: "Multiple operations",
    intent_source: "test",
    confidence_level: 0.99
  }
  DataSemantics {
    state: {
      items: Array<String>,
      count: Integer = 0
    },
    invariants: ["count >= 0"]
  }
  BehavioralSemantics {
    operations: [
      {
        name: "add_item",
        precondition: "item_not_duplicate",
        parameters: {
          item: String
        },
        postcondition: "item_added",
        side_effects: ["log_addition"],
        idempotence: "not_idempotent"
      },
      {
        name: "clear_all",
        precondition: "items_not_empty",
        parameters: {},
        postcondition: "items_empty",
        side_effects: ["log_clear"],
        idempotence: "idempotent"
      }
    ]
  }
  ExecutionConstraints {
    trigger_types: ["manual", "event_based"],
    resource_limits: {
      max_memory_bytes: 2097152,
      computation_timeout_ms: 200,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }
  HumanMachineContract {
    system_commitments: ["Items managed correctly"],
    system_refusals: ["No duplicate items"],
    user_obligations: ["May add or remove items"]
  }
}"#;
        let ast = parse_valid(input);
        assert_eq!(ast.behavioral_semantics.operations.len(), 2);

        let op1 = &ast.behavioral_semantics.operations[0];
        assert_eq!(op1.name.value, "add_item");
        assert_eq!(op1.parameters.len(), 1);
        assert_eq!(op1.parameters[0].name.value, "item");

        let op2 = &ast.behavioral_semantics.operations[1];
        assert_eq!(op2.name.value, "clear_all");
        assert_eq!(op2.parameters.len(), 0);
    }

    // ── Extensions ─────────────────────────────────────

    #[test]
    fn test_parse_with_extensions() {
        let input = format!("{}\n\nExtensions {{\n  custom_system {{\n    priority: \"high\",\n    tags: [\"experimental\", \"beta\"]\n  }}\n}}", MINIMAL_CONTRACT);
        let ast = parse_valid(&input);

        let ext = ast.extensions.as_ref().expect("Expected extensions");
        assert_eq!(ext.systems.len(), 1);
        assert_eq!(ext.systems[0].name.value, "custom_system");
        assert_eq!(ext.systems[0].fields.len(), 2);
        assert_eq!(ext.systems[0].fields[0].name.value, "priority");

        if let LiteralValue::Array(items, _) = &ext.systems[0].fields[1].value {
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected array value for tags");
        }
    }

    // ── Invalid inputs ─────────────────────────────────

    #[test]
    fn test_parse_missing_identity() {
        let input = r#"Contract {
  PurposeStatement {
    narrative: "No identity",
    intent_source: "test",
    confidence_level: 1.0
  }
}"#;
        let err = parse_err(input);
        assert!(err.contains("Expected Identity"), "Error: {}", err);
    }

    #[test]
    fn test_parse_missing_closing_brace() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
"#;
        let err = parse_err(input);
        // Should fail: missing closing brace means it expects PurposeStatement but hits Eof
        assert!(
            err.contains("Expected") || err.contains("found"),
            "Error: {}",
            err
        );
    }

    #[test]
    fn test_parse_wrong_version_type() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-test-001",
    version: "not_an_integer",
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
}"#;
        let err = parse_err(input);
        assert!(err.contains("Expected integer literal"), "Error: {}", err);
    }

    #[test]
    fn test_parse_confidence_out_of_range() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
  PurposeStatement {
    narrative: "Invalid confidence",
    intent_source: "test",
    confidence_level: 2.5
  }
}"#;
        let err = parse_err(input);
        assert!(err.contains("confidence_level"), "Error: {}", err);
    }

    #[test]
    fn test_parse_unknown_section() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
  FakeSection {
    something: "invalid"
  }
}"#;
        let err = parse_err(input);
        // Parser expects PurposeStatement, finds Identifier("FakeSection")
        assert!(
            err.contains("Expected") || err.contains("PurposeStatement"),
            "Error: {}",
            err
        );
    }

    // ── Conformance fixtures (file-based) ──────────────

    fn read_fixture(path: &str) -> String {
        let full = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(path);
        fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", full.display(), e))
    }

    #[test]
    fn test_conformance_valid_minimal_contract() {
        let input = read_fixture("conformance/valid/minimal-contract.icl");
        let ast = parse_valid(&input);
        assert_eq!(ast.identity.stable_id.value, "ic-test-001");
    }

    #[test]
    fn test_conformance_valid_all_primitive_types() {
        let input = read_fixture("conformance/valid/all-primitive-types.icl");
        let ast = parse_valid(&input);
        assert_eq!(ast.data_semantics.state.len(), 6);
    }

    #[test]
    fn test_conformance_valid_composite_types() {
        let input = read_fixture("conformance/valid/composite-types.icl");
        let ast = parse_valid(&input);
        assert_eq!(ast.data_semantics.state.len(), 4);
    }

    #[test]
    fn test_conformance_valid_multiple_operations() {
        let input = read_fixture("conformance/valid/multiple-operations.icl");
        let ast = parse_valid(&input);
        assert_eq!(ast.behavioral_semantics.operations.len(), 3);
    }

    #[test]
    fn test_conformance_valid_with_extensions() {
        let input = read_fixture("conformance/valid/with-extensions.icl");
        let ast = parse_valid(&input);
        assert!(ast.extensions.is_some());
    }

    #[test]
    fn test_conformance_invalid_missing_identity() {
        let input = read_fixture("conformance/invalid/missing-identity.icl");
        assert!(parse(&input).is_err());
    }

    #[test]
    fn test_conformance_invalid_missing_closing_brace() {
        let input = read_fixture("conformance/invalid/missing-closing-brace.icl");
        assert!(parse(&input).is_err());
    }

    #[test]
    fn test_conformance_invalid_wrong_version_type() {
        let input = read_fixture("conformance/invalid/wrong-version-type.icl");
        assert!(parse(&input).is_err());
    }

    #[test]
    fn test_conformance_invalid_confidence_out_of_range() {
        let input = read_fixture("conformance/invalid/confidence-out-of-range.icl");
        assert!(parse(&input).is_err());
    }

    #[test]
    fn test_conformance_invalid_unknown_section() {
        let input = read_fixture("conformance/invalid/unknown-section.icl");
        assert!(parse(&input).is_err());
    }

    // ── Determinism proof ──────────────────────────────

    #[test]
    fn test_parse_determinism_100_iterations() {
        let first = parse(MINIMAL_CONTRACT).unwrap();

        for i in 0..100 {
            let result = parse(MINIMAL_CONTRACT).unwrap();
            assert_eq!(first, result, "Determinism failure at iteration {}", i);
        }
    }

    #[test]
    fn test_parse_determinism_complex_contract() {
        let input = read_fixture("conformance/valid/all-primitive-types.icl");
        let first = parse(&input).unwrap();

        for i in 0..100 {
            let result = parse(&input).unwrap();
            assert_eq!(first, result, "Determinism failure at iteration {}", i);
        }
    }

    // ── Empty input / edge cases ───────────────────────

    #[test]
    fn test_parse_empty_input() {
        assert!(parse("").is_err());
    }

    #[test]
    fn test_parse_just_contract_keyword() {
        assert!(parse("Contract").is_err());
    }

    #[test]
    fn test_parse_empty_state() {
        // Contract with empty state: {} should parse
        let input = r#"Contract {
  Identity {
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
  PurposeStatement {
    narrative: "Empty state",
    intent_source: "test",
    confidence_level: 0.5
  }
  DataSemantics {
    state: {},
    invariants: []
  }
  BehavioralSemantics {
    operations: []
  }
  ExecutionConstraints {
    trigger_types: ["manual"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }
  HumanMachineContract {
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }
}"#;
        let ast = parse_valid(input);
        assert_eq!(ast.data_semantics.state.len(), 0);
    }
}
