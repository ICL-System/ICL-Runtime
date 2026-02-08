//! Contract verifier — checks types, invariants, determinism, and coherence
//!
//! The verifier ensures contracts are valid before execution.
//! It checks: types, invariant consistency, determinism, and structural coherence.
//!
//! # Architecture
//!
//! The verifier operates on the AST (`ContractNode`) to preserve type information
//! and source spans for error reporting. It accumulates all diagnostics rather
//! than stopping at the first error, giving users a complete picture.
//!
//! # Verification Phases (per spec §4.1)
//!
//! 1. **Type Correctness** — All types well-formed, defaults match declared types
//! 2. **Invariant Consistency** — Invariants reference valid state fields
//! 3. **Determinism** — No non-deterministic patterns detected
//! 4. **Coherence** — Structural validity (unique names, valid ranges, feasible limits)

use std::collections::BTreeSet;

use crate::parser::ast::*;
use crate::parser::tokenizer::Span;

// ── Verification Result Types ─────────────────────────────

/// Result of contract verification — accumulates all diagnostics
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub diagnostics: Vec<Diagnostic>,
}

impl VerificationResult {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Returns true if no errors were found (warnings are OK)
    pub fn is_valid(&self) -> bool {
        !self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    /// Returns only error-level diagnostics
    pub fn errors(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .collect()
    }

    /// Returns only warning-level diagnostics
    pub fn warnings(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .collect()
    }

    fn add_error(&mut self, kind: DiagnosticKind, message: String, span: Option<Span>) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            kind,
            message,
            span,
        });
    }

    fn add_warning(&mut self, kind: DiagnosticKind, message: String, span: Option<Span>) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            kind,
            message,
            span,
        });
    }
}

impl Default for VerificationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// A single verification diagnostic
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Option<Span>,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let prefix = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        if let Some(ref span) = self.span {
            write!(f, "{} [{}] at {}: {}", prefix, self.kind, span, self.message)
        } else {
            write!(f, "{} [{}]: {}", prefix, self.kind, self.message)
        }
    }
}

/// Severity level for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// Category of verification issue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    TypeError,
    InvariantError,
    DeterminismViolation,
    CoherenceError,
}

impl std::fmt::Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DiagnosticKind::TypeError => write!(f, "type"),
            DiagnosticKind::InvariantError => write!(f, "invariant"),
            DiagnosticKind::DeterminismViolation => write!(f, "determinism"),
            DiagnosticKind::CoherenceError => write!(f, "coherence"),
        }
    }
}

// ── Public API ────────────────────────────────────────────

/// Verify a parsed contract AST for correctness.
///
/// Runs all verification phases and returns accumulated diagnostics.
/// Does not stop at first error — reports everything found.
pub fn verify(ast: &ContractNode) -> VerificationResult {
    let mut result = VerificationResult::new();

    // Phase 3.1 — Type Checker
    verify_types(ast, &mut result);

    // Phase 3.2 — Invariant Verifier
    verify_invariants(ast, &mut result);

    // Phase 3.3 — Determinism Checker
    verify_determinism(ast, &mut result);

    // Phase 3.4 — Coherence Verifier
    verify_coherence(ast, &mut result);

    result
}

// ── Phase 3.1: Type Checker ──────────────────────────────

/// Validate all types in the contract are well-formed and defaults match declared types.
fn verify_types(ast: &ContractNode, result: &mut VerificationResult) {
    // Check Identity constraints
    verify_identity_types(&ast.identity, result);

    // Check PurposeStatement constraints
    verify_purpose_types(&ast.purpose_statement, result);

    // Check state field types
    for field in &ast.data_semantics.state {
        verify_type_expression(&field.type_expr, result);
        if let Some(ref default) = field.default_value {
            verify_default_matches_type(
                &field.name.value,
                &field.type_expr,
                default,
                result,
            );
        }
    }

    // Check operation parameter types
    for op in &ast.behavioral_semantics.operations {
        for param in &op.parameters {
            verify_type_expression(&param.type_expr, result);
            if let Some(ref default) = param.default_value {
                verify_default_matches_type(
                    &param.name.value,
                    &param.type_expr,
                    default,
                    result,
                );
            }
        }
    }

    // Check resource limits are valid
    verify_resource_limit_types(&ast.execution_constraints.resource_limits, result);
}

/// Verify Identity field constraints (spec §1.2)
fn verify_identity_types(identity: &IdentityNode, result: &mut VerificationResult) {
    // Version must be non-negative
    if identity.version.value < 0 {
        result.add_error(
            DiagnosticKind::TypeError,
            format!("version must be non-negative, found {}", identity.version.value),
            Some(identity.version.span.clone()),
        );
    }

    // stable_id must match pattern: [a-z0-9][a-z0-9\-]{0,30}[a-z0-9]
    let sid = &identity.stable_id.value;
    if !is_valid_stable_id(sid) {
        result.add_error(
            DiagnosticKind::TypeError,
            format!(
                "stable_id '{}' does not match required pattern [a-z0-9][a-z0-9-]{{0,30}}[a-z0-9]",
                sid
            ),
            Some(identity.stable_id.span.clone()),
        );
    }

    // semantic_hash must be valid hex
    let hash = &identity.semantic_hash.value;
    if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        result.add_error(
            DiagnosticKind::TypeError,
            format!("semantic_hash '{}' is not valid hexadecimal", hash),
            Some(identity.semantic_hash.span.clone()),
        );
    }
}

/// Check if a stable_id matches the spec pattern
fn is_valid_stable_id(id: &str) -> bool {
    if id.len() < 2 || id.len() > 32 {
        return false;
    }
    let bytes = id.as_bytes();
    // First and last must be [a-z0-9]
    let valid_alnum = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
    let valid_middle = |b: u8| valid_alnum(b) || b == b'-';
    if !valid_alnum(bytes[0]) || !valid_alnum(bytes[bytes.len() - 1]) {
        return false;
    }
    bytes[1..bytes.len() - 1].iter().all(|&b| valid_middle(b))
}

/// Verify PurposeStatement constraints (spec §1.3)
fn verify_purpose_types(purpose: &PurposeStatementNode, result: &mut VerificationResult) {
    // confidence_level must be in [0.0, 1.0]
    let cl = purpose.confidence_level.value;
    if !(0.0..=1.0).contains(&cl) {
        result.add_error(
            DiagnosticKind::TypeError,
            format!("confidence_level must be in range [0.0, 1.0], found {}", cl),
            Some(purpose.confidence_level.span.clone()),
        );
    }

    // narrative should be < 500 chars (warning, not error)
    if purpose.narrative.value.len() > 500 {
        result.add_warning(
            DiagnosticKind::TypeError,
            format!(
                "narrative exceeds recommended 500 character limit ({} chars)",
                purpose.narrative.value.len()
            ),
            Some(purpose.narrative.span.clone()),
        );
    }
}

/// Verify a type expression is well-formed
fn verify_type_expression(type_expr: &TypeExpression, result: &mut VerificationResult) {
    match type_expr {
        TypeExpression::Primitive(_, _) => {
            // All primitive types are valid by construction
        }
        TypeExpression::Array(inner, _) => {
            verify_type_expression(inner, result);
        }
        TypeExpression::Map(key, value, span) => {
            // Map keys must be a hashable/comparable type
            verify_type_expression(key, result);
            verify_type_expression(value, result);
            verify_map_key_type(key, span, result);
        }
        TypeExpression::Object(fields, _) => {
            // Check for duplicate field names
            let mut seen = BTreeSet::new();
            for field in fields {
                if !seen.insert(&field.name.value) {
                    result.add_error(
                        DiagnosticKind::TypeError,
                        format!("duplicate field name '{}' in Object type", field.name.value),
                        Some(field.name.span.clone()),
                    );
                }
                verify_type_expression(&field.type_expr, result);
                if let Some(ref default) = field.default_value {
                    verify_default_matches_type(
                        &field.name.value,
                        &field.type_expr,
                        default,
                        result,
                    );
                }
            }
        }
        TypeExpression::Enum(variants, span) => {
            // Enum must have at least one variant
            if variants.is_empty() {
                result.add_error(
                    DiagnosticKind::TypeError,
                    "Enum type must have at least one variant".to_string(),
                    Some(span.clone()),
                );
            }
            // Enum variants must be unique
            let mut seen = BTreeSet::new();
            for variant in variants {
                if !seen.insert(&variant.value) {
                    result.add_error(
                        DiagnosticKind::TypeError,
                        format!("duplicate Enum variant '{}'", variant.value),
                        Some(variant.span.clone()),
                    );
                }
            }
        }
    }
}

/// Verify Map key type is a valid key type (must be hashable/comparable)
fn verify_map_key_type(
    key_type: &TypeExpression,
    map_span: &Span,
    result: &mut VerificationResult,
) {
    match key_type {
        TypeExpression::Primitive(pt, _) => match pt {
            PrimitiveType::String
            | PrimitiveType::Integer
            | PrimitiveType::Uuid
            | PrimitiveType::Boolean
            | PrimitiveType::Iso8601 => {
                // Valid key types
            }
            PrimitiveType::Float => {
                result.add_error(
                    DiagnosticKind::TypeError,
                    "Float cannot be used as Map key type (non-deterministic equality)".to_string(),
                    Some(map_span.clone()),
                );
            }
        },
        TypeExpression::Enum(_, _) => {
            // Enum is a valid key type (string-based)
        }
        _ => {
            result.add_error(
                DiagnosticKind::TypeError,
                format!(
                    "Map key type must be a primitive or Enum, found {}",
                    type_expr_name(key_type)
                ),
                Some(map_span.clone()),
            );
        }
    }
}

/// Verify a default value matches its declared type
fn verify_default_matches_type(
    field_name: &str,
    type_expr: &TypeExpression,
    default: &LiteralValue,
    result: &mut VerificationResult,
) {
    let matches = default_matches_type(type_expr, default);
    if !matches {
        result.add_error(
            DiagnosticKind::TypeError,
            format!(
                "default value for '{}' has type {}, expected {}",
                field_name,
                literal_type_name(default),
                type_expr_name(type_expr),
            ),
            Some(literal_span(default)),
        );
    }
}

/// Check if a literal value is compatible with a type expression
fn default_matches_type(type_expr: &TypeExpression, default: &LiteralValue) -> bool {
    match (type_expr, default) {
        (TypeExpression::Primitive(PrimitiveType::Integer, _), LiteralValue::Integer(_, _)) => true,
        (TypeExpression::Primitive(PrimitiveType::Float, _), LiteralValue::Float(_, _)) => true,
        // Allow integer literals as float defaults (e.g., 0 for Float)
        (TypeExpression::Primitive(PrimitiveType::Float, _), LiteralValue::Integer(_, _)) => true,
        (TypeExpression::Primitive(PrimitiveType::String, _), LiteralValue::String(_, _)) => true,
        (TypeExpression::Primitive(PrimitiveType::Boolean, _), LiteralValue::Boolean(_, _)) => true,
        // ISO8601 and UUID are typically string literals
        (TypeExpression::Primitive(PrimitiveType::Iso8601, _), LiteralValue::String(_, _)) => true,
        (TypeExpression::Primitive(PrimitiveType::Uuid, _), LiteralValue::String(_, _)) => true,
        // Enum default must be a string that matches a variant
        (TypeExpression::Enum(variants, _), LiteralValue::String(s, _)) => {
            variants.iter().any(|v| v.value == *s)
        }
        // Array default must be array of matching elements
        (TypeExpression::Array(elem_type, _), LiteralValue::Array(elems, _)) => {
            elems.iter().all(|e| default_matches_type(elem_type, e))
        }
        _ => false,
    }
}

/// Verify resource limits are valid positive values
fn verify_resource_limit_types(limits: &ResourceLimitsNode, result: &mut VerificationResult) {
    if limits.max_memory_bytes.value <= 0 {
        result.add_error(
            DiagnosticKind::TypeError,
            format!(
                "max_memory_bytes must be positive, found {}",
                limits.max_memory_bytes.value
            ),
            Some(limits.max_memory_bytes.span.clone()),
        );
    }
    if limits.computation_timeout_ms.value <= 0 {
        result.add_error(
            DiagnosticKind::TypeError,
            format!(
                "computation_timeout_ms must be positive, found {}",
                limits.computation_timeout_ms.value
            ),
            Some(limits.computation_timeout_ms.span.clone()),
        );
    }
    if limits.max_state_size_bytes.value <= 0 {
        result.add_error(
            DiagnosticKind::TypeError,
            format!(
                "max_state_size_bytes must be positive, found {}",
                limits.max_state_size_bytes.value
            ),
            Some(limits.max_state_size_bytes.span.clone()),
        );
    }
}

// ── Phase 3.2: Invariant Verifier ─────────────────────────

/// Verify invariants reference valid state fields and are logically consistent.
fn verify_invariants(ast: &ContractNode, result: &mut VerificationResult) {
    let state_field_names: BTreeSet<&str> = ast
        .data_semantics
        .state
        .iter()
        .map(|f| f.name.value.as_str())
        .collect();

    for invariant in &ast.data_semantics.invariants {
        let inv_text = &invariant.value;

        // Extract potential field references from invariant text
        let referenced_fields = extract_identifiers(inv_text);

        // Check that referenced identifiers that look like field names exist in state
        let mut found_field_ref = false;
        for ident in &referenced_fields {
            if state_field_names.contains(ident.as_str()) {
                found_field_ref = true;
            }
        }

        // Warn if invariant doesn't reference any state fields
        if !found_field_ref && !state_field_names.is_empty() && !inv_text.is_empty() {
            result.add_warning(
                DiagnosticKind::InvariantError,
                format!(
                    "invariant '{}' does not reference any declared state fields",
                    inv_text,
                ),
                Some(invariant.span.clone()),
            );
        }
    }

    // Check for duplicate invariants
    let mut seen = BTreeSet::new();
    for invariant in &ast.data_semantics.invariants {
        if !seen.insert(&invariant.value) {
            result.add_warning(
                DiagnosticKind::InvariantError,
                format!("duplicate invariant: '{}'", invariant.value),
                Some(invariant.span.clone()),
            );
        }
    }
}

/// Extract identifiers (potential field references) from an invariant/condition string
fn extract_identifiers(text: &str) -> Vec<String> {
    let mut identifiers = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_alphabetic() || c == '_' {
            let mut ident = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    ident.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            // Filter out common keywords/comparators — keep likely field names
            if !is_keyword(&ident) {
                identifiers.push(ident);
            }
        } else {
            chars.next();
        }
    }

    identifiers
}

/// Check if identifier is a common keyword (not a field reference)
fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "is" | "not" | "and" | "or" | "true" | "false" | "null" | "empty"
            | "if" | "then" | "else" | "for" | "while" | "in"
            | "gt" | "lt" | "eq" | "ne" | "ge" | "le"
            | "the" | "a" | "an" | "of" | "to" | "at" | "by"
            | "must" | "should" | "can" | "may" | "will"
            | "exists" | "unique" | "valid" | "always" | "never"
            | "updated" | "set" | "contains" | "matches"
    )
}

// ── Phase 3.3: Determinism Checker ────────────────────────

/// Check for non-deterministic patterns in contract text fields.
fn verify_determinism(ast: &ContractNode, result: &mut VerificationResult) {
    // Patterns that suggest non-determinism
    let nondeterministic_patterns = [
        // Randomness
        ("random", "randomness usage"),
        ("rand(", "random function call"),
        ("Math.random", "random function call"),
        ("uuid_generate", "runtime UUID generation"),
        ("generate_id", "runtime ID generation"),
        // System time
        ("now()", "system time access"),
        ("current_time", "system time access"),
        ("system_time", "system time access"),
        ("Date.now", "system time access"),
        ("time.time", "system time access"),
        ("Instant::now", "system time access"),
        // External I/O
        ("fetch(", "external I/O"),
        ("http_request", "external I/O"),
        ("read_file", "external I/O"),
        ("write_file", "external I/O"),
        ("network_call", "external I/O"),
        ("socket", "external I/O"),
        // Hash iteration
        ("HashMap", "non-deterministic hash iteration"),
        ("HashSet", "non-deterministic hash iteration"),
        ("dict_keys", "non-deterministic hash iteration"),
    ];

    // Check operation preconditions, postconditions, side_effects
    for op in &ast.behavioral_semantics.operations {
        check_string_for_nondeterminism(
            &op.precondition.value,
            &format!("operation '{}' precondition", op.name.value),
            &op.precondition.span,
            &nondeterministic_patterns,
            result,
        );
        check_string_for_nondeterminism(
            &op.postcondition.value,
            &format!("operation '{}' postcondition", op.name.value),
            &op.postcondition.span,
            &nondeterministic_patterns,
            result,
        );
        for se in &op.side_effects {
            check_string_for_nondeterminism(
                &se.value,
                &format!("operation '{}' side_effect", op.name.value),
                &se.span,
                &nondeterministic_patterns,
                result,
            );
        }
        check_string_for_nondeterminism(
            &op.idempotence.value,
            &format!("operation '{}' idempotence", op.name.value),
            &op.idempotence.span,
            &nondeterministic_patterns,
            result,
        );
    }

    // Check invariants for non-deterministic patterns
    for inv in &ast.data_semantics.invariants {
        check_string_for_nondeterminism(
            &inv.value,
            "invariant",
            &inv.span,
            &nondeterministic_patterns,
            result,
        );
    }
}

/// Check a string for non-deterministic patterns
fn check_string_for_nondeterminism(
    text: &str,
    context: &str,
    span: &Span,
    patterns: &[(&str, &str)],
    result: &mut VerificationResult,
) {
    let lower = text.to_lowercase();
    for &(pattern, description) in patterns {
        if lower.contains(&pattern.to_lowercase()) {
            result.add_error(
                DiagnosticKind::DeterminismViolation,
                format!(
                    "{} detected in {}: text contains '{}'",
                    description, context, pattern,
                ),
                Some(span.clone()),
            );
        }
    }
}

// ── Phase 3.4: Coherence Verifier ─────────────────────────

/// Check structural coherence of the contract.
fn verify_coherence(ast: &ContractNode, result: &mut VerificationResult) {
    // Check unique operation names
    verify_unique_operation_names(ast, result);

    // Check unique state field names
    verify_unique_state_fields(ast, result);

    // Check sandbox_mode is a known value
    verify_sandbox_mode(ast, result);

    // Check trigger_types are known values
    verify_trigger_types(ast, result);

    // Check operations reference valid state fields in pre/postconditions
    verify_operation_field_references(ast, result);

    // Check extension namespace isolation
    verify_extension_namespaces(ast, result);
}

/// Verify operation names are unique
fn verify_unique_operation_names(ast: &ContractNode, result: &mut VerificationResult) {
    let mut seen = BTreeSet::new();
    for op in &ast.behavioral_semantics.operations {
        if !seen.insert(&op.name.value) {
            result.add_error(
                DiagnosticKind::CoherenceError,
                format!("duplicate operation name '{}'", op.name.value),
                Some(op.name.span.clone()),
            );
        }
    }
}

/// Verify state field names are unique
fn verify_unique_state_fields(ast: &ContractNode, result: &mut VerificationResult) {
    let mut seen = BTreeSet::new();
    for field in &ast.data_semantics.state {
        if !seen.insert(&field.name.value) {
            result.add_error(
                DiagnosticKind::CoherenceError,
                format!("duplicate state field name '{}'", field.name.value),
                Some(field.name.span.clone()),
            );
        }
    }
}

/// Verify sandbox_mode is a recognized value
fn verify_sandbox_mode(ast: &ContractNode, result: &mut VerificationResult) {
    let valid_modes = ["full_isolation", "restricted", "none"];
    let mode = &ast.execution_constraints.sandbox_mode.value;
    if !valid_modes.contains(&mode.as_str()) {
        result.add_warning(
            DiagnosticKind::CoherenceError,
            format!(
                "unrecognized sandbox_mode '{}', expected one of: {}",
                mode,
                valid_modes.join(", ")
            ),
            Some(ast.execution_constraints.sandbox_mode.span.clone()),
        );
    }
}

/// Verify trigger_types contain recognized values
fn verify_trigger_types(ast: &ContractNode, result: &mut VerificationResult) {
    let valid_types = ["manual", "time_based", "event_based"];
    for tt in &ast.execution_constraints.trigger_types {
        if !valid_types.contains(&tt.value.as_str()) {
            result.add_warning(
                DiagnosticKind::CoherenceError,
                format!(
                    "unrecognized trigger_type '{}', expected one of: {}",
                    tt.value,
                    valid_types.join(", ")
                ),
                Some(tt.span.clone()),
            );
        }
    }
}

/// Verify operation pre/postconditions reference valid state fields
fn verify_operation_field_references(ast: &ContractNode, result: &mut VerificationResult) {
    let state_field_names: BTreeSet<&str> = ast
        .data_semantics
        .state
        .iter()
        .map(|f| f.name.value.as_str())
        .collect();

    for op in &ast.behavioral_semantics.operations {
        // Check precondition references
        let pre_idents = extract_identifiers(&op.precondition.value);
        for ident in &pre_idents {
            if looks_like_field_ref(ident) && !state_field_names.contains(ident.as_str()) {
                // Only warn — preconditions may reference parameters too
                let param_names: BTreeSet<&str> =
                    op.parameters.iter().map(|p| p.name.value.as_str()).collect();
                if !param_names.contains(ident.as_str()) {
                    result.add_warning(
                        DiagnosticKind::CoherenceError,
                        format!(
                            "precondition of '{}' references unknown field '{}'",
                            op.name.value, ident,
                        ),
                        Some(op.precondition.span.clone()),
                    );
                }
            }
        }

        // Check postcondition references
        let post_idents = extract_identifiers(&op.postcondition.value);
        for ident in &post_idents {
            if looks_like_field_ref(ident) && !state_field_names.contains(ident.as_str()) {
                let param_names: BTreeSet<&str> =
                    op.parameters.iter().map(|p| p.name.value.as_str()).collect();
                if !param_names.contains(ident.as_str()) {
                    result.add_warning(
                        DiagnosticKind::CoherenceError,
                        format!(
                            "postcondition of '{}' references unknown field '{}'",
                            op.name.value, ident,
                        ),
                        Some(op.postcondition.span.clone()),
                    );
                }
            }
        }
    }
}

/// Check if an identifier looks like a field reference (snake_case, not a common word)
fn looks_like_field_ref(ident: &str) -> bool {
    // Must be lowercase with underscores, at least 2 chars
    ident.len() >= 2
        && ident.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && ident.contains('_')
}

/// Verify extension namespaces are unique
fn verify_extension_namespaces(ast: &ContractNode, result: &mut VerificationResult) {
    if let Some(ref ext) = ast.extensions {
        let mut seen = BTreeSet::new();
        for system in &ext.systems {
            if !seen.insert(&system.name.value) {
                result.add_error(
                    DiagnosticKind::CoherenceError,
                    format!("duplicate extension namespace '{}'", system.name.value),
                    Some(system.name.span.clone()),
                );
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────

/// Human-readable name for a type expression
fn type_expr_name(type_expr: &TypeExpression) -> String {
    match type_expr {
        TypeExpression::Primitive(pt, _) => pt.to_string(),
        TypeExpression::Array(inner, _) => format!("Array<{}>", type_expr_name(inner)),
        TypeExpression::Map(k, v, _) => {
            format!("Map<{}, {}>", type_expr_name(k), type_expr_name(v))
        }
        TypeExpression::Object(_, _) => "Object".to_string(),
        TypeExpression::Enum(_, _) => "Enum".to_string(),
    }
}

/// Human-readable name for a literal value type
fn literal_type_name(lit: &LiteralValue) -> String {
    match lit {
        LiteralValue::String(_, _) => "String".to_string(),
        LiteralValue::Integer(_, _) => "Integer".to_string(),
        LiteralValue::Float(_, _) => "Float".to_string(),
        LiteralValue::Boolean(_, _) => "Boolean".to_string(),
        LiteralValue::Array(_, _) => "Array".to_string(),
    }
}

/// Get the span of a literal value
fn literal_span(lit: &LiteralValue) -> Span {
    match lit {
        LiteralValue::String(_, s) => s.clone(),
        LiteralValue::Integer(_, s) => s.clone(),
        LiteralValue::Float(_, s) => s.clone(),
        LiteralValue::Boolean(_, s) => s.clone(),
        LiteralValue::Array(_, s) => s.clone(),
    }
}

// ── Tests ─────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    // ── Helper: parse and verify ──────────────────────────

    fn parse_and_verify(input: &str) -> VerificationResult {
        let ast = parse(input).expect("test input should parse");
        verify(&ast)
    }

    // ── Phase 3.1: Type Checker Tests ─────────────────────

    #[test]
    fn test_valid_minimal_contract() {
        let result = parse_and_verify(include_str!(
            "../../../../ICL-Spec/conformance/valid/minimal-contract.icl"
        ));
        assert!(result.is_valid(), "minimal contract should verify: {:?}", result.errors());
    }

    #[test]
    fn test_valid_all_primitive_types() {
        let result = parse_and_verify(include_str!(
            "../../../../ICL-Spec/conformance/valid/all-primitive-types.icl"
        ));
        assert!(result.is_valid(), "all-primitive-types should verify: {:?}", result.errors());
    }

    #[test]
    fn test_valid_composite_types() {
        let result = parse_and_verify(include_str!(
            "../../../../ICL-Spec/conformance/valid/composite-types.icl"
        ));
        assert!(result.is_valid(), "composite-types should verify: {:?}", result.errors());
    }

    #[test]
    fn test_valid_multiple_operations() {
        let result = parse_and_verify(include_str!(
            "../../../../ICL-Spec/conformance/valid/multiple-operations.icl"
        ));
        assert!(result.is_valid(), "multiple-operations should verify: {:?}", result.errors());
    }

    #[test]
    fn test_integer_type_valid_default() {
        let input = make_contract_with_state("count: Integer = 42");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "Integer default 42 should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_float_type_valid_default() {
        let input = make_contract_with_state("ratio: Float = 3.14");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "Float default 3.14 should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_float_type_integer_default_allowed() {
        let input = make_contract_with_state("ratio: Float = 0");
        let result = parse_and_verify(&input);
        assert!(
            result.is_valid(),
            "Integer literal as Float default should be allowed: {:?}",
            result.errors()
        );
    }

    #[test]
    fn test_string_type_valid_default() {
        let input = make_contract_with_state("label: String = \"hello\"");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "String default should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_boolean_type_valid_default() {
        let input = make_contract_with_state("active: Boolean = true");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "Boolean default should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_type_mismatch_string_for_integer() {
        let input = make_contract_with_state("count: Integer = \"hello\"");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "String default for Integer should fail");
        assert!(
            result.errors().iter().any(|d| d.kind == DiagnosticKind::TypeError),
            "Should produce TypeError"
        );
    }

    #[test]
    fn test_type_mismatch_integer_for_string() {
        let input = make_contract_with_state("label: String = 42");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "Integer default for String should fail");
    }

    #[test]
    fn test_type_mismatch_boolean_for_integer() {
        let input = make_contract_with_state("count: Integer = true");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "Boolean default for Integer should fail");
    }

    #[test]
    fn test_type_mismatch_string_for_boolean() {
        let input = make_contract_with_state("active: Boolean = \"yes\"");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "String default for Boolean should fail");
    }

    #[test]
    fn test_enum_valid_default() {
        let input = make_contract_with_state("status: Enum [\"active\", \"inactive\"] = \"active\"");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "Valid Enum default should pass: {:?}", result.errors());
    }

    #[test]
    fn test_enum_invalid_default() {
        let input =
            make_contract_with_state("status: Enum [\"active\", \"inactive\"] = \"unknown\"");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "Invalid Enum default should fail");
    }

    #[test]
    fn test_enum_duplicate_variants() {
        let input =
            make_contract_with_state("status: Enum [\"active\", \"active\", \"inactive\"]");
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.message.contains("duplicate Enum variant")),
            "Should detect duplicate Enum variants"
        );
    }

    #[test]
    fn test_object_duplicate_fields() {
        let input = make_contract_with_state(
            "data: Object { name: String, name: Integer }",
        );
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.message.contains("duplicate field name")),
            "Should detect duplicate Object fields"
        );
    }

    #[test]
    fn test_map_float_key_rejected() {
        let input = make_contract_with_state("lookup: Map<Float, String>");
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.message.contains("Float cannot be used as Map key")),
            "Float Map keys should be rejected"
        );
    }

    #[test]
    fn test_map_string_key_valid() {
        let input = make_contract_with_state("lookup: Map<String, Integer>");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "String Map key should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_array_type_valid() {
        let input = make_contract_with_state("items: Array<String>");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "Array<String> should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_nested_collection_types() {
        let input = make_contract_with_state("matrix: Array<Array<Integer>>");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "Nested Array should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_confidence_level_out_of_range_high() {
        // Parser already validates confidence_level in [0.0, 1.0]
        let input = make_contract_with_confidence("1.5");
        assert!(parse(&input).is_err(), "confidence_level 1.5 should fail at parse");
    }

    #[test]
    fn test_confidence_level_out_of_range_low() {
        // Verifier catches out-of-range on a constructed AST
        let mut ast = make_valid_ast();
        ast.purpose_statement.confidence_level = SpannedValue::new(-0.1, dummy_span());
        let result = verify(&ast);
        assert!(!result.is_valid(), "confidence_level -0.1 should fail");
        assert!(
            result.errors().iter().any(|d| d.message.contains("confidence_level")),
            "Should mention confidence_level: {:?}",
            result.errors()
        );
    }

    #[test]
    fn test_confidence_level_boundary_zero() {
        let input = make_contract_with_confidence("0.0");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "confidence_level 0.0 should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_confidence_level_boundary_one() {
        let input = make_contract_with_confidence("1.0");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "confidence_level 1.0 should be valid: {:?}", result.errors());
    }

    #[test]
    fn test_negative_version() {
        // Verifier catches negative version on constructed AST
        let mut ast = make_valid_ast();
        ast.identity.version = SpannedValue::new(-1, dummy_span());
        let result = verify(&ast);
        assert!(!result.is_valid(), "negative version should fail");
        assert!(
            result.errors().iter().any(|d| d.message.contains("version")),
            "Should mention version: {:?}",
            result.errors()
        );
    }

    #[test]
    fn test_negative_resource_limits() {
        // Verifier catches negative resource limits on constructed AST
        let mut ast = make_valid_ast();
        ast.execution_constraints.resource_limits.max_memory_bytes =
            SpannedValue::new(-1, dummy_span());
        let result = verify(&ast);
        assert!(!result.is_valid(), "negative max_memory_bytes should fail");
        assert!(
            result.errors().iter().any(|d| d.message.contains("max_memory_bytes")),
            "Should mention max_memory_bytes: {:?}",
            result.errors()
        );
    }

    #[test]
    fn test_zero_timeout() {
        let input = make_contract_with_resource_limits(1048576, 0, 1048576);
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "zero computation_timeout_ms should fail");
    }

    #[test]
    fn test_valid_stable_id() {
        let input = make_contract_with_stable_id("ic-test-001");
        let result = parse_and_verify(&input);
        assert!(result.is_valid(), "valid stable_id: {:?}", result.errors());
    }

    #[test]
    fn test_invalid_stable_id_uppercase() {
        let input = make_contract_with_stable_id("IC-TEST-001");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "uppercase stable_id should fail");
    }

    #[test]
    fn test_invalid_stable_id_starts_with_dash() {
        let input = make_contract_with_stable_id("-invalid");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "dash-start stable_id should fail");
    }

    #[test]
    fn test_invalid_semantic_hash() {
        let input = make_contract_with_hash("not-hex-at-all!");
        let result = parse_and_verify(&input);
        assert!(!result.is_valid(), "non-hex hash should fail");
    }

    // ── Phase 3.2: Invariant Verifier Tests ───────────────

    #[test]
    fn test_invariant_references_valid_field() {
        let input = make_contract_with_state_and_invariants(
            "count: Integer = 0",
            &["count >= 0"],
        );
        let result = parse_and_verify(&input);
        // Should not warn about unreferenced fields
        assert!(
            !result.warnings().iter().any(|d| d.kind == DiagnosticKind::InvariantError),
            "Valid field reference should not warn: {:?}",
            result.warnings()
        );
    }

    #[test]
    fn test_duplicate_invariant_warning() {
        let input = make_contract_with_state_and_invariants(
            "count: Integer = 0",
            &["count >= 0", "count >= 0"],
        );
        let result = parse_and_verify(&input);
        assert!(
            result.warnings().iter().any(|d| d.message.contains("duplicate invariant")),
            "Should warn about duplicate invariants"
        );
    }

    // ── Phase 3.3: Determinism Checker Tests ──────────────

    #[test]
    fn test_detect_randomness_in_precondition() {
        let input = make_contract_with_operation(
            "random_op",
            "random() > 0.5",
            "result set",
        );
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.kind == DiagnosticKind::DeterminismViolation),
            "Should detect randomness: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_detect_system_time_in_postcondition() {
        let input = make_contract_with_operation(
            "time_op",
            "true",
            "timestamp = now()",
        );
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.kind == DiagnosticKind::DeterminismViolation),
            "Should detect system time: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_detect_external_io() {
        let input = make_contract_with_operation(
            "io_op",
            "true",
            "data = fetch(url)",
        );
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.kind == DiagnosticKind::DeterminismViolation),
            "Should detect external I/O: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_detect_hashmap_usage() {
        let input = make_contract_with_operation(
            "hash_op",
            "true",
            "HashMap iteration order",
        );
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.kind == DiagnosticKind::DeterminismViolation),
            "Should detect HashMap: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_clean_operation_no_determinism_violation() {
        let input = make_contract_with_operation(
            "clean_op",
            "count >= 0",
            "count updated",
        );
        let result = parse_and_verify(&input);
        assert!(
            !result.errors().iter().any(|d| d.kind == DiagnosticKind::DeterminismViolation),
            "Clean operation should have no determinism violations: {:?}",
            result.errors()
        );
    }

    // ── Phase 3.4: Coherence Verifier Tests ───────────────

    #[test]
    fn test_duplicate_operation_names() {
        let input = make_contract_with_two_ops("update_count", "update_count");
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.message.contains("duplicate operation name")),
            "Should detect duplicate operation names: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_unique_operation_names() {
        let input = make_contract_with_two_ops("create_item", "delete_item");
        let result = parse_and_verify(&input);
        assert!(
            !result.errors().iter().any(|d| d.message.contains("duplicate operation name")),
            "Unique operation names should pass: {:?}",
            result.errors()
        );
    }

    #[test]
    fn test_duplicate_state_fields() {
        let input = make_contract_with_state("count: Integer, count: String");
        let result = parse_and_verify(&input);
        assert!(
            result.errors().iter().any(|d| d.message.contains("duplicate state field")),
            "Should detect duplicate state fields: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_unknown_sandbox_mode_warning() {
        let input = make_contract_with_sandbox_mode("super_isolated");
        let result = parse_and_verify(&input);
        assert!(
            result.warnings().iter().any(|d| d.message.contains("sandbox_mode")),
            "Unknown sandbox_mode should warn: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_valid_sandbox_modes() {
        for mode in &["full_isolation", "restricted", "none"] {
            let input = make_contract_with_sandbox_mode(mode);
            let result = parse_and_verify(&input);
            assert!(
                !result.warnings().iter().any(|d| d.message.contains("sandbox_mode")),
                "sandbox_mode '{}' should not warn: {:?}",
                mode,
                result.warnings()
            );
        }
    }

    #[test]
    fn test_unknown_trigger_type_warning() {
        let input = make_contract_with_trigger_types(&["cron_job"]);
        let result = parse_and_verify(&input);
        assert!(
            result.warnings().iter().any(|d| d.message.contains("trigger_type")),
            "Unknown trigger_type should warn: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn test_valid_trigger_types() {
        let input = make_contract_with_trigger_types(&["manual", "time_based", "event_based"]);
        let result = parse_and_verify(&input);
        assert!(
            !result.warnings().iter().any(|d| d.message.contains("trigger_type")),
            "Known trigger_types should not warn: {:?}",
            result.warnings()
        );
    }

    // ── Conformance Suite ─────────────────────────────────

    #[test]
    fn test_conformance_valid_all_pass_verification() {
        let fixtures = [
            include_str!("../../../../ICL-Spec/conformance/valid/minimal-contract.icl"),
            include_str!("../../../../ICL-Spec/conformance/valid/all-primitive-types.icl"),
            include_str!("../../../../ICL-Spec/conformance/valid/composite-types.icl"),
            include_str!("../../../../ICL-Spec/conformance/valid/multiple-operations.icl"),
        ];
        for (i, fixture) in fixtures.iter().enumerate() {
            let result = parse_and_verify(fixture);
            assert!(
                result.is_valid(),
                "conformance fixture {} should verify: {:?}",
                i,
                result.errors()
            );
        }
    }

    // ── Determinism Tests ─────────────────────────────────

    #[test]
    fn test_verification_determinism_100_iterations() {
        let input = include_str!("../../../../ICL-Spec/conformance/valid/all-primitive-types.icl");
        let ast = parse(input).expect("should parse");

        let first = verify(&ast);
        let first_count = first.diagnostics.len();
        let first_valid = first.is_valid();

        for i in 0..100 {
            let result = verify(&ast);
            assert_eq!(
                result.diagnostics.len(),
                first_count,
                "Determinism failure at iteration {}: diagnostic count differs",
                i
            );
            assert_eq!(
                result.is_valid(),
                first_valid,
                "Determinism failure at iteration {}: validity differs",
                i
            );
            // Compare each diagnostic message
            for (j, (a, b)) in first.diagnostics.iter().zip(result.diagnostics.iter()).enumerate() {
                assert_eq!(
                    a.message, b.message,
                    "Determinism failure at iteration {}, diagnostic {}: messages differ",
                    i, j
                );
                assert_eq!(
                    a.severity, b.severity,
                    "Determinism failure at iteration {}, diagnostic {}: severities differ",
                    i, j
                );
            }
        }
    }

    #[test]
    fn test_verification_determinism_complex_contract() {
        let input = include_str!("../../../../ICL-Spec/conformance/valid/multiple-operations.icl");
        let ast = parse(input).expect("should parse");

        let first = verify(&ast);
        for i in 0..100 {
            let result = verify(&ast);
            assert_eq!(
                result.diagnostics.len(),
                first.diagnostics.len(),
                "Determinism failure at iteration {} on complex contract",
                i
            );
        }
    }

    // ── Test Helpers ──────────────────────────────────────

    fn make_contract_with_state(state_fields: &str) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      {}
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            state_fields
        )
    }

    fn make_contract_with_state_and_invariants(state_fields: &str, invariants: &[&str]) -> String {
        let inv_str = invariants
            .iter()
            .map(|i| format!("\"{}\"", i))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      {}
    }},
    invariants: [{}]
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            state_fields, inv_str
        )
    }

    fn make_contract_with_confidence(level: &str) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: {}
  }}
  DataSemantics {{
    state: {{
      value: String
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            level
        )
    }

    fn make_contract_with_resource_limits(mem: i64, timeout: i64, state: i64) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      value: String
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: {},
      computation_timeout_ms: {},
      max_state_size_bytes: {}
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            mem, timeout, state
        )
    }

    fn make_contract_with_stable_id(id: &str) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "{}",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      value: String
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            id
        )
    }

    fn make_contract_with_hash(hash: &str) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "{}"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      value: String
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            hash
        )
    }

    fn make_contract_with_operation(name: &str, precondition: &str, postcondition: &str) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      count: Integer = 0,
      result: String
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: [
      {{
        name: "{}",
        precondition: "{}",
        parameters: {{}},
        postcondition: "{}",
        side_effects: [],
        idempotence: "idempotent"
      }}
    ]
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            name, precondition, postcondition
        )
    }

    fn make_contract_with_two_ops(name1: &str, name2: &str) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      count: Integer = 0
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: [
      {{
        name: "{}",
        precondition: "true",
        parameters: {{}},
        postcondition: "done",
        side_effects: [],
        idempotence: "idempotent"
      }},
      {{
        name: "{}",
        precondition: "true",
        parameters: {{}},
        postcondition: "done",
        side_effects: [],
        idempotence: "idempotent"
      }}
    ]
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            name1, name2
        )
    }

    fn make_contract_with_sandbox_mode(mode: &str) -> String {
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      value: String
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: ["manual"],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "{}"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            mode
        )
    }

    fn make_contract_with_trigger_types(types: &[&str]) -> String {
        let types_str = types
            .iter()
            .map(|t| format!("\"{}\"", t))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            r#"Contract {{
  Identity {{
    stable_id: "ic-test-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }}
  PurposeStatement {{
    narrative: "Test contract",
    intent_source: "test",
    confidence_level: 1.0
  }}
  DataSemantics {{
    state: {{
      value: String
    }},
    invariants: []
  }}
  BehavioralSemantics {{
    operations: []
  }}
  ExecutionConstraints {{
    trigger_types: [{}],
    resource_limits: {{
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    }},
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }}
  HumanMachineContract {{
    system_commitments: [],
    system_refusals: [],
    user_obligations: []
  }}
}}"#,
            types_str
        )
    }

    /// Create a dummy span for AST construction in tests
    fn dummy_span() -> Span {
        Span { line: 0, column: 0, offset: 0 }
    }

    /// Create a minimal valid AST for direct manipulation in tests
    fn make_valid_ast() -> ContractNode {
        ContractNode {
            identity: IdentityNode {
                stable_id: SpannedValue::new("ic-test-001".to_string(), dummy_span()),
                version: SpannedValue::new(1, dummy_span()),
                created_timestamp: SpannedValue::new(
                    "2026-02-01T00:00:00Z".to_string(),
                    dummy_span(),
                ),
                owner: SpannedValue::new("test".to_string(), dummy_span()),
                semantic_hash: SpannedValue::new("0000000000000000".to_string(), dummy_span()),
                span: dummy_span(),
            },
            purpose_statement: PurposeStatementNode {
                narrative: SpannedValue::new("Test contract".to_string(), dummy_span()),
                intent_source: SpannedValue::new("test".to_string(), dummy_span()),
                confidence_level: SpannedValue::new(1.0, dummy_span()),
                span: dummy_span(),
            },
            data_semantics: DataSemanticsNode {
                state: vec![StateFieldNode {
                    name: SpannedValue::new("value".to_string(), dummy_span()),
                    type_expr: TypeExpression::Primitive(PrimitiveType::String, dummy_span()),
                    default_value: None,
                    span: dummy_span(),
                }],
                invariants: vec![],
                span: dummy_span(),
            },
            behavioral_semantics: BehavioralSemanticsNode {
                operations: vec![],
                span: dummy_span(),
            },
            execution_constraints: ExecutionConstraintsNode {
                trigger_types: vec![SpannedValue::new("manual".to_string(), dummy_span())],
                resource_limits: ResourceLimitsNode {
                    max_memory_bytes: SpannedValue::new(1048576, dummy_span()),
                    computation_timeout_ms: SpannedValue::new(100, dummy_span()),
                    max_state_size_bytes: SpannedValue::new(1048576, dummy_span()),
                    span: dummy_span(),
                },
                external_permissions: vec![],
                sandbox_mode: SpannedValue::new("full_isolation".to_string(), dummy_span()),
                span: dummy_span(),
            },
            human_machine_contract: HumanMachineContractNode {
                system_commitments: vec![],
                system_refusals: vec![],
                user_obligations: vec![],
                span: dummy_span(),
            },
            extensions: None,
            span: dummy_span(),
        }
    }
}
