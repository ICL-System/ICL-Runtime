//! Canonical normalizer — converts ICL to deterministic canonical form
//!
//! The normalizer transforms an ICL contract into its canonical representation.
//! This is the single deterministic form used for hashing, comparison, and storage.
//!
//! # Pipeline
//!
//! `ICL text → parse → AST → normalize_ast → serialize_canonical → SHA-256`
//!
//! # Guarantees
//!
//! - **Idempotent**: `normalize(normalize(x)) == normalize(x)`
//! - **Deterministic**: same input always produces same output
//! - **Unique**: each distinct contract has one canonical form
//! - **Semantic preserving**: no information loss

use sha2::{Digest, Sha256};

use crate::parser::ast::*;
use crate::parser::tokenizer::Span;
use crate::Result;

// ── Public API ─────────────────────────────────────────────

/// Normalize ICL text to canonical form
///
/// Pipeline: parse → normalize AST → serialize → compute hash
///
/// # Guarantees
/// - Idempotent: `normalize(normalize(x)) == normalize(x)`
/// - Deterministic: same input always produces same output
/// - Semantic preserving: `parse(normalize(x))` preserves all meaning
///
/// # Errors
/// Returns `ParseError` for invalid input or `NormalizationError`
/// if the contract cannot be canonicalized.
pub fn normalize(icl: &str) -> Result<String> {
    let ast = crate::parser::parse(icl)?;
    let normalized = normalize_ast(ast);
    let canonical = serialize_canonical(&normalized);
    Ok(canonical)
}

/// Normalize a parsed AST to canonical form (sorted, expanded, hashed)
///
/// Steps per CORE-SPECIFICATION.md §6.1:
/// 1. Sort state fields alphabetically
/// 2. Sort operation parameters alphabetically
/// 3. Sort operations by name
/// 4. Sort string lists alphabetically
/// 5. Expand defaults (already in AST)
/// 6. Compute SHA-256 semantic hash
pub fn normalize_ast(mut ast: ContractNode) -> ContractNode {
    // ── Step 1: Sort state fields ──────────────────────
    ast.data_semantics
        .state
        .sort_by(|a, b| a.name.value.cmp(&b.name.value));

    // Sort Object type fields recursively
    for field in &mut ast.data_semantics.state {
        normalize_type_fields(&mut field.type_expr);
    }

    // ── Step 2: Sort invariants ────────────────────────
    ast.data_semantics
        .invariants
        .sort_by(|a, b| a.value.cmp(&b.value));

    // ── Step 3: Sort operations by name ────────────────
    ast.behavioral_semantics
        .operations
        .sort_by(|a, b| a.name.value.cmp(&b.name.value));

    // ── Step 4: Sort operation internals ───────────────
    for op in &mut ast.behavioral_semantics.operations {
        op.parameters
            .sort_by(|a, b| a.name.value.cmp(&b.name.value));
        for param in &mut op.parameters {
            normalize_type_fields(&mut param.type_expr);
        }
        op.side_effects.sort_by(|a, b| a.value.cmp(&b.value));
    }

    // ── Step 5: Sort string lists ──────────────────────
    ast.execution_constraints
        .trigger_types
        .sort_by(|a, b| a.value.cmp(&b.value));
    ast.execution_constraints
        .external_permissions
        .sort_by(|a, b| a.value.cmp(&b.value));
    ast.human_machine_contract
        .system_commitments
        .sort_by(|a, b| a.value.cmp(&b.value));
    ast.human_machine_contract
        .system_refusals
        .sort_by(|a, b| a.value.cmp(&b.value));
    ast.human_machine_contract
        .user_obligations
        .sort_by(|a, b| a.value.cmp(&b.value));

    // ── Step 6: Sort extensions ────────────────────────
    if let Some(ref mut ext) = ast.extensions {
        ext.systems.sort_by(|a, b| a.name.value.cmp(&b.name.value));
        for sys in &mut ext.systems {
            sys.fields.sort_by(|a, b| a.name.value.cmp(&b.name.value));
        }
    }

    // ── Step 7: Compute semantic hash ──────────────────
    // Hash is computed over the canonical form *excluding* the semantic_hash field
    let hash = compute_semantic_hash(&ast);
    ast.identity.semantic_hash = SpannedValue::new(hash, dummy_span());

    ast
}

/// Normalize a parsed Contract struct to canonical form
pub fn normalize_contract(contract: &crate::Contract) -> Result<crate::Contract> {
    // Round-trip: Contract → serialize to ICL text → parse → normalize → lower
    // This ensures we use the canonical pipeline
    let text = serialize_contract_to_icl(contract);
    let normalized_text = normalize(&text)?;
    crate::parser::parse_contract(&normalized_text)
}

// ── Canonical Serializer ───────────────────────────────────

/// Serialize a ContractNode AST to canonical ICL text
///
/// Produces deterministic output with:
/// - Fixed section order (Identity, PurposeStatement, etc.)
/// - 2-space indentation
/// - One field per line
/// - No comments
/// - Consistent formatting
pub fn serialize_canonical(ast: &ContractNode) -> String {
    let mut out = String::new();

    out.push_str("Contract {\n");
    serialize_identity(&mut out, &ast.identity);
    serialize_purpose_statement(&mut out, &ast.purpose_statement);
    serialize_data_semantics(&mut out, &ast.data_semantics);
    serialize_behavioral_semantics(&mut out, &ast.behavioral_semantics);
    serialize_execution_constraints(&mut out, &ast.execution_constraints);
    serialize_human_machine_contract(&mut out, &ast.human_machine_contract);
    out.push_str("}\n");

    if let Some(ref ext) = ast.extensions {
        out.push('\n');
        serialize_extensions(&mut out, ext);
    }

    out
}

// ── Section serializers ────────────────────────────────────

fn serialize_identity(out: &mut String, id: &IdentityNode) {
    out.push_str("  Identity {\n");
    write_field_str(out, 4, "created_timestamp", &id.created_timestamp.value);
    write_field_str(out, 4, "owner", &id.owner.value);
    write_field_str(out, 4, "semantic_hash", &id.semantic_hash.value);
    write_field_str(out, 4, "stable_id", &id.stable_id.value);
    write_field_int(out, 4, "version", id.version.value);
    out.push_str("  }\n");
}

fn serialize_purpose_statement(out: &mut String, ps: &PurposeStatementNode) {
    out.push_str("  PurposeStatement {\n");
    write_field_float(out, 4, "confidence_level", ps.confidence_level.value);
    write_field_str(out, 4, "intent_source", &ps.intent_source.value);
    write_field_str(out, 4, "narrative", &ps.narrative.value);
    out.push_str("  }\n");
}

fn serialize_data_semantics(out: &mut String, ds: &DataSemanticsNode) {
    out.push_str("  DataSemantics {\n");
    write_indent(out, 4);
    out.push_str("invariants: ");
    serialize_string_list(out, &ds.invariants);
    out.push_str(",\n");
    write_indent(out, 4);
    out.push_str("state: {\n");
    for field in &ds.state {
        serialize_state_field(out, field, 6);
    }
    write_indent(out, 4);
    out.push_str("}\n");
    out.push_str("  }\n");
}

fn serialize_behavioral_semantics(out: &mut String, bs: &BehavioralSemanticsNode) {
    out.push_str("  BehavioralSemantics {\n");
    write_indent(out, 4);
    out.push_str("operations: [\n");
    for (i, op) in bs.operations.iter().enumerate() {
        serialize_operation(out, op, 6);
        if i < bs.operations.len() - 1 {
            // Comma between operations handled by the comma after }
        }
    }
    write_indent(out, 4);
    out.push_str("]\n");
    out.push_str("  }\n");
}

fn serialize_operation(out: &mut String, op: &OperationNode, indent: usize) {
    write_indent(out, indent);
    out.push_str("{\n");
    write_field_str(out, indent + 2, "idempotence", &op.idempotence.value);
    write_field_str(out, indent + 2, "name", &op.name.value);
    // parameters
    write_indent(out, indent + 2);
    out.push_str("parameters: {\n");
    for param in &op.parameters {
        serialize_state_field(out, param, indent + 4);
    }
    write_indent(out, indent + 2);
    out.push_str("},\n");
    write_field_str(out, indent + 2, "postcondition", &op.postcondition.value);
    write_field_str(out, indent + 2, "precondition", &op.precondition.value);
    write_indent(out, indent + 2);
    out.push_str("side_effects: ");
    serialize_string_list(out, &op.side_effects);
    out.push('\n');
    write_indent(out, indent);
    out.push_str("}\n");
}

fn serialize_execution_constraints(out: &mut String, ec: &ExecutionConstraintsNode) {
    out.push_str("  ExecutionConstraints {\n");
    write_indent(out, 4);
    out.push_str("external_permissions: ");
    serialize_string_list(out, &ec.external_permissions);
    out.push_str(",\n");

    // resource_limits
    write_indent(out, 4);
    out.push_str("resource_limits: {\n");
    write_field_int(
        out,
        6,
        "computation_timeout_ms",
        ec.resource_limits.computation_timeout_ms.value,
    );
    write_field_int(
        out,
        6,
        "max_memory_bytes",
        ec.resource_limits.max_memory_bytes.value,
    );
    write_field_int(
        out,
        6,
        "max_state_size_bytes",
        ec.resource_limits.max_state_size_bytes.value,
    );
    write_indent(out, 4);
    out.push_str("},\n");

    write_field_str(out, 4, "sandbox_mode", &ec.sandbox_mode.value);
    write_indent(out, 4);
    out.push_str("trigger_types: ");
    serialize_string_list(out, &ec.trigger_types);
    out.push('\n');
    out.push_str("  }\n");
}

fn serialize_human_machine_contract(out: &mut String, hmc: &HumanMachineContractNode) {
    out.push_str("  HumanMachineContract {\n");
    write_indent(out, 4);
    out.push_str("system_commitments: ");
    serialize_string_list(out, &hmc.system_commitments);
    out.push_str(",\n");
    write_indent(out, 4);
    out.push_str("system_refusals: ");
    serialize_string_list(out, &hmc.system_refusals);
    out.push_str(",\n");
    write_indent(out, 4);
    out.push_str("user_obligations: ");
    serialize_string_list(out, &hmc.user_obligations);
    out.push('\n');
    out.push_str("  }\n");
}

fn serialize_extensions(out: &mut String, ext: &ExtensionsNode) {
    out.push_str("Extensions {\n");
    for sys in &ext.systems {
        write_indent(out, 2);
        out.push_str(&sys.name.value);
        out.push_str(" {\n");
        for field in &sys.fields {
            write_indent(out, 4);
            out.push_str(&field.name.value);
            out.push_str(": ");
            serialize_literal_value(out, &field.value);
            out.push('\n');
        }
        write_indent(out, 2);
        out.push_str("}\n");
    }
    out.push_str("}\n");
}

// ── Field serializers ──────────────────────────────────────

fn serialize_state_field(out: &mut String, field: &StateFieldNode, indent: usize) {
    write_indent(out, indent);
    out.push_str(&field.name.value);
    out.push_str(": ");
    serialize_type_expression(out, &field.type_expr);
    if let Some(ref default) = field.default_value {
        out.push_str(" = ");
        serialize_literal_value(out, default);
    }
    out.push_str(",\n");
}

fn serialize_type_expression(out: &mut String, ty: &TypeExpression) {
    match ty {
        TypeExpression::Primitive(p, _) => out.push_str(&p.to_string()),
        TypeExpression::Array(inner, _) => {
            out.push_str("Array<");
            serialize_type_expression(out, inner);
            out.push('>');
        }
        TypeExpression::Map(k, v, _) => {
            out.push_str("Map<");
            serialize_type_expression(out, k);
            out.push_str(", ");
            serialize_type_expression(out, v);
            out.push('>');
        }
        TypeExpression::Object(fields, _) => {
            out.push_str("Object {\n");
            // Fields already sorted during normalization
            for f in fields {
                // Increase indent by context — we use a fixed deeper indent
                out.push_str("        "); // 8 spaces for nested object fields
                out.push_str(&f.name.value);
                out.push_str(": ");
                serialize_type_expression(out, &f.type_expr);
                if let Some(ref def) = f.default_value {
                    out.push_str(" = ");
                    serialize_literal_value(out, def);
                }
                out.push_str(",\n");
            }
            out.push_str("      }"); // Close at parent indent
        }
        TypeExpression::Enum(variants, _) => {
            out.push_str("Enum[");
            for (i, v) in variants.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push('"');
                out.push_str(&v.value);
                out.push('"');
            }
            out.push(']');
        }
    }
}

fn serialize_literal_value(out: &mut String, val: &LiteralValue) {
    match val {
        LiteralValue::String(s, _) => {
            out.push('"');
            out.push_str(s);
            out.push('"');
        }
        LiteralValue::Integer(n, _) => out.push_str(&n.to_string()),
        LiteralValue::Float(f, _) => {
            // Ensure we always have a decimal point
            let s = format!("{}", f);
            if s.contains('.') {
                out.push_str(&s);
            } else {
                out.push_str(&format!("{}.0", f));
            }
        }
        LiteralValue::Boolean(b, _) => out.push_str(if *b { "true" } else { "false" }),
        LiteralValue::Array(items, _) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                serialize_literal_value(out, item);
            }
            out.push(']');
        }
    }
}

fn serialize_string_list(out: &mut String, items: &[SpannedValue<String>]) {
    out.push('[');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(&item.value);
        out.push('"');
    }
    out.push(']');
}

// ── Helpers ────────────────────────────────────────────────

fn write_indent(out: &mut String, n: usize) {
    for _ in 0..n {
        out.push(' ');
    }
}

fn write_field_str(out: &mut String, indent: usize, name: &str, value: &str) {
    write_indent(out, indent);
    out.push_str(name);
    out.push_str(": \"");
    out.push_str(value);
    out.push_str("\",\n");
}

fn write_field_int(out: &mut String, indent: usize, name: &str, value: i64) {
    write_indent(out, indent);
    out.push_str(name);
    out.push_str(": ");
    out.push_str(&value.to_string());
    out.push_str(",\n");
}

fn write_field_float(out: &mut String, indent: usize, name: &str, value: f64) {
    write_indent(out, indent);
    out.push_str(name);
    out.push_str(": ");
    let s = format!("{}", value);
    if s.contains('.') {
        out.push_str(&s);
    } else {
        out.push_str(&format!("{}.0", value));
    }
    out.push_str(",\n");
}

fn normalize_type_fields(ty: &mut TypeExpression) {
    match ty {
        TypeExpression::Object(fields, _) => {
            fields.sort_by(|a, b| a.name.value.cmp(&b.name.value));
            for f in fields.iter_mut() {
                normalize_type_fields(&mut f.type_expr);
            }
        }
        TypeExpression::Array(inner, _) => normalize_type_fields(inner),
        TypeExpression::Map(k, v, _) => {
            normalize_type_fields(k);
            normalize_type_fields(v);
        }
        TypeExpression::Enum(variants, _) => {
            // Sort enum variants alphabetically for canonical form
            variants.sort_by(|a, b| a.value.cmp(&b.value));
        }
        TypeExpression::Primitive(_, _) => {}
    }
}

fn dummy_span() -> Span {
    Span {
        line: 0,
        column: 0,
        offset: 0,
    }
}

// ── SHA-256 Hash Computation ──────────────────────────────

/// Compute SHA-256 semantic hash of a normalized AST
///
/// The hash is computed over the canonical serialization
/// with the semantic_hash field set to a placeholder value.
/// This ensures the hash doesn't include itself.
pub fn compute_semantic_hash(ast: &ContractNode) -> String {
    // Clone AST with a placeholder hash
    let mut hashable = ast.clone();
    hashable.identity.semantic_hash = SpannedValue::new(
        "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        dummy_span(),
    );

    let canonical = serialize_canonical(&hashable);
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

// ── Contract ↔ ICL text helpers ────────────────────────────

/// Serialize a Contract struct to ICL text (for round-tripping through normalizer)
fn serialize_contract_to_icl(contract: &crate::Contract) -> String {
    let mut out = String::new();
    out.push_str("Contract {\n");

    // Identity
    out.push_str("  Identity {\n");
    write_field_str(&mut out, 4, "stable_id", &contract.identity.stable_id);
    write_field_int(&mut out, 4, "version", contract.identity.version as i64);
    write_field_str(
        &mut out,
        4,
        "created_timestamp",
        &contract.identity.created_timestamp,
    );
    write_field_str(&mut out, 4, "owner", &contract.identity.owner);
    write_field_str(
        &mut out,
        4,
        "semantic_hash",
        &contract.identity.semantic_hash,
    );
    out.push_str("  }\n");

    // PurposeStatement
    out.push_str("  PurposeStatement {\n");
    write_field_str(
        &mut out,
        4,
        "narrative",
        &contract.purpose_statement.narrative,
    );
    write_field_str(
        &mut out,
        4,
        "intent_source",
        &contract.purpose_statement.intent_source,
    );
    write_field_float(
        &mut out,
        4,
        "confidence_level",
        contract.purpose_statement.confidence_level,
    );
    out.push_str("  }\n");

    // DataSemantics — state as empty since Contract uses serde_json::Value
    out.push_str("  DataSemantics {\n");
    out.push_str("    state: {},\n");
    write_indent(&mut out, 4);
    out.push_str("invariants: [");
    for (i, inv) in contract.data_semantics.invariants.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(inv);
        out.push('"');
    }
    out.push_str("]\n");
    out.push_str("  }\n");

    // BehavioralSemantics
    out.push_str("  BehavioralSemantics {\n");
    out.push_str("    operations: [\n");
    for op in &contract.behavioral_semantics.operations {
        out.push_str("      {\n");
        write_field_str(&mut out, 8, "name", &op.name);
        write_field_str(&mut out, 8, "precondition", &op.precondition);
        out.push_str("        parameters: {},\n");
        write_field_str(&mut out, 8, "postcondition", &op.postcondition);
        write_indent(&mut out, 8);
        out.push_str("side_effects: [");
        for (i, se) in op.side_effects.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push('"');
            out.push_str(se);
            out.push('"');
        }
        out.push_str("],\n");
        write_field_str(&mut out, 8, "idempotence", &op.idempotence);
        out.push_str("      }\n");
    }
    out.push_str("    ]\n");
    out.push_str("  }\n");

    // ExecutionConstraints
    out.push_str("  ExecutionConstraints {\n");
    write_indent(&mut out, 4);
    out.push_str("trigger_types: [");
    for (i, t) in contract
        .execution_constraints
        .trigger_types
        .iter()
        .enumerate()
    {
        if i > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(t);
        out.push('"');
    }
    out.push_str("],\n");
    out.push_str("    resource_limits: {\n");
    write_field_int(
        &mut out,
        6,
        "max_memory_bytes",
        contract
            .execution_constraints
            .resource_limits
            .max_memory_bytes as i64,
    );
    write_field_int(
        &mut out,
        6,
        "computation_timeout_ms",
        contract
            .execution_constraints
            .resource_limits
            .computation_timeout_ms as i64,
    );
    write_field_int(
        &mut out,
        6,
        "max_state_size_bytes",
        contract
            .execution_constraints
            .resource_limits
            .max_state_size_bytes as i64,
    );
    out.push_str("    },\n");
    write_indent(&mut out, 4);
    out.push_str("external_permissions: [");
    for (i, p) in contract
        .execution_constraints
        .external_permissions
        .iter()
        .enumerate()
    {
        if i > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(p);
        out.push('"');
    }
    out.push_str("],\n");
    write_field_str(
        &mut out,
        4,
        "sandbox_mode",
        &contract.execution_constraints.sandbox_mode,
    );
    out.push_str("  }\n");

    // HumanMachineContract
    out.push_str("  HumanMachineContract {\n");
    write_string_list(
        &mut out,
        4,
        "system_commitments",
        &contract.human_machine_contract.system_commitments,
    );
    write_string_list(
        &mut out,
        4,
        "system_refusals",
        &contract.human_machine_contract.system_refusals,
    );
    write_string_list(
        &mut out,
        4,
        "user_obligations",
        &contract.human_machine_contract.user_obligations,
    );
    out.push_str("  }\n");

    out.push_str("}\n");
    out
}

fn write_string_list(out: &mut String, indent: usize, name: &str, items: &[String]) {
    write_indent(out, indent);
    out.push_str(name);
    out.push_str(": [");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(item);
        out.push('"');
    }
    out.push_str("],\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

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

    fn read_fixture(path: &str) -> String {
        let full = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(path);
        fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", full.display(), e))
    }

    // ── Basic normalization ────────────────────────────

    #[test]
    fn test_normalize_minimal_contract() {
        let result = normalize(MINIMAL_CONTRACT).unwrap();
        assert!(result.contains("Contract {"));
        assert!(result.contains("Identity {"));
        assert!(result.contains("semantic_hash:"));
    }

    #[test]
    fn test_normalize_produces_valid_icl() {
        // Normalized output must parse successfully
        let normalized = normalize(MINIMAL_CONTRACT).unwrap();
        let ast = crate::parser::parse(&normalized);
        assert!(
            ast.is_ok(),
            "Normalized output doesn't parse: {:?}",
            ast.err()
        );
    }

    // ── Sorting ────────────────────────────────────────

    #[test]
    fn test_normalize_sorts_state_fields() {
        // State fields: z_field, a_field — should be sorted to a_field, z_field
        let input = r#"Contract {
  Identity {
    stable_id: "ic-sort-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
  PurposeStatement {
    narrative: "Sort test",
    intent_source: "test",
    confidence_level: 0.5
  }
  DataSemantics {
    state: {
      z_field: String,
      a_field: Integer
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
        let normalized = normalize(input).unwrap();
        let a_pos = normalized.find("a_field").unwrap();
        let z_pos = normalized.find("z_field").unwrap();
        assert!(
            a_pos < z_pos,
            "a_field should come before z_field in normalized output"
        );
    }

    #[test]
    fn test_normalize_sorts_operations_by_name() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-sort-ops-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
  PurposeStatement {
    narrative: "Sort ops test",
    intent_source: "test",
    confidence_level: 0.5
  }
  DataSemantics {
    state: {},
    invariants: []
  }
  BehavioralSemantics {
    operations: [
      {
        name: "z_operation",
        precondition: "none",
        parameters: {},
        postcondition: "done",
        side_effects: [],
        idempotence: "idempotent"
      },
      {
        name: "a_operation",
        precondition: "none",
        parameters: {},
        postcondition: "done",
        side_effects: [],
        idempotence: "idempotent"
      }
    ]
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
        let normalized = normalize(input).unwrap();
        let a_pos = normalized.find("a_operation").unwrap();
        let z_pos = normalized.find("z_operation").unwrap();
        assert!(a_pos < z_pos, "a_operation should come before z_operation");
    }

    #[test]
    fn test_normalize_sorts_string_lists() {
        let input = r#"Contract {
  Identity {
    stable_id: "ic-sort-lists-001",
    version: 1,
    created_timestamp: 2026-02-01T00:00:00Z,
    owner: "test",
    semantic_hash: "0000000000000000"
  }
  PurposeStatement {
    narrative: "Sort lists test",
    intent_source: "test",
    confidence_level: 0.5
  }
  DataSemantics {
    state: {},
    invariants: ["z_invariant", "a_invariant"]
  }
  BehavioralSemantics {
    operations: []
  }
  ExecutionConstraints {
    trigger_types: ["z_trigger", "a_trigger"],
    resource_limits: {
      max_memory_bytes: 1048576,
      computation_timeout_ms: 100,
      max_state_size_bytes: 1048576
    },
    external_permissions: [],
    sandbox_mode: "full_isolation"
  }
  HumanMachineContract {
    system_commitments: ["z_commit", "a_commit"],
    system_refusals: [],
    user_obligations: []
  }
}"#;
        let normalized = normalize(input).unwrap();

        // Check invariants sorted
        let a_inv = normalized.find("a_invariant").unwrap();
        let z_inv = normalized.find("z_invariant").unwrap();
        assert!(a_inv < z_inv, "Invariants should be sorted");

        // Check trigger_types sorted
        let a_trig = normalized.find("a_trigger").unwrap();
        let z_trig = normalized.find("z_trigger").unwrap();
        assert!(a_trig < z_trig, "Trigger types should be sorted");

        // Check commitments sorted
        let a_com = normalized.find("a_commit").unwrap();
        let z_com = normalized.find("z_commit").unwrap();
        assert!(a_com < z_com, "Commitments should be sorted");
    }

    // ── Canonical field order in Identity ──────────────

    #[test]
    fn test_normalize_identity_fields_sorted() {
        let normalized = normalize(MINIMAL_CONTRACT).unwrap();
        // In canonical form, Identity fields should be alphabetical:
        // created_timestamp, owner, semantic_hash, stable_id, version
        let ct = normalized.find("created_timestamp").unwrap();
        let ow = normalized.find("owner").unwrap();
        let sh = normalized.find("semantic_hash").unwrap();
        let si = normalized.find("stable_id").unwrap();
        let ver = normalized.find("version").unwrap();
        assert!(ct < ow, "created_timestamp before owner");
        assert!(ow < sh, "owner before semantic_hash");
        assert!(sh < si, "semantic_hash before stable_id");
        assert!(si < ver, "stable_id before version");
    }

    // ── Comment removal ────────────────────────────────

    #[test]
    fn test_normalize_removes_comments() {
        let input = format!("// This is a comment\n{}", MINIMAL_CONTRACT);
        let normalized = normalize(&input).unwrap();
        assert!(!normalized.contains("// This is a comment"));
    }

    // ── SHA-256 hash ───────────────────────────────────

    #[test]
    fn test_normalize_computes_sha256_hash() {
        let normalized = normalize(MINIMAL_CONTRACT).unwrap();
        let ast = crate::parser::parse(&normalized).unwrap();
        let hash = &ast.identity.semantic_hash.value;

        // SHA-256 hex is 64 chars
        assert_eq!(hash.len(), 64, "Hash should be 64 hex chars, got: {}", hash);
        assert!(
            hash.chars().all(|c| c.is_ascii_hexdigit()),
            "Hash should be hex, got: {}",
            hash
        );
    }

    #[test]
    fn test_normalize_hash_is_deterministic() {
        let hash1 = {
            let n = normalize(MINIMAL_CONTRACT).unwrap();
            let ast = crate::parser::parse(&n).unwrap();
            ast.identity.semantic_hash.value
        };
        let hash2 = {
            let n = normalize(MINIMAL_CONTRACT).unwrap();
            let ast = crate::parser::parse(&n).unwrap();
            ast.identity.semantic_hash.value
        };
        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_different_contracts_different_hashes() {
        let contract_a = MINIMAL_CONTRACT;
        let contract_b = MINIMAL_CONTRACT.replace("ic-test-001", "ic-test-002");

        let hash_a = {
            let n = normalize(contract_a).unwrap();
            let ast = crate::parser::parse(&n).unwrap();
            ast.identity.semantic_hash.value
        };
        let hash_b = {
            let n = normalize(&contract_b).unwrap();
            let ast = crate::parser::parse(&n).unwrap();
            ast.identity.semantic_hash.value
        };
        assert_ne!(
            hash_a, hash_b,
            "Different contracts should have different hashes"
        );
    }

    // ── Idempotence proof ──────────────────────────────

    #[test]
    fn test_idempotence() {
        let once = normalize(MINIMAL_CONTRACT).unwrap();
        let twice = normalize(&once).unwrap();
        assert_eq!(
            once, twice,
            "normalize(normalize(x)) must equal normalize(x)"
        );
    }

    #[test]
    fn test_idempotence_complex_contract() {
        let input = read_fixture("conformance/valid/all-primitive-types.icl");
        let once = normalize(&input).unwrap();
        let twice = normalize(&once).unwrap();
        assert_eq!(once, twice, "Idempotence failure on complex contract");
    }

    #[test]
    fn test_idempotence_with_operations() {
        let input = read_fixture("conformance/valid/multiple-operations.icl");
        let once = normalize(&input).unwrap();
        let twice = normalize(&once).unwrap();
        assert_eq!(
            once, twice,
            "Idempotence failure on contract with operations"
        );
    }

    #[test]
    fn test_idempotence_with_extensions() {
        let input = read_fixture("conformance/valid/with-extensions.icl");
        let once = normalize(&input).unwrap();
        let twice = normalize(&once).unwrap();
        assert_eq!(
            once, twice,
            "Idempotence failure on contract with extensions"
        );
    }

    // ── Determinism proof (100 iterations) ─────────────

    #[test]
    fn test_determinism_100_iterations() {
        let first = normalize(MINIMAL_CONTRACT).unwrap();

        for i in 0..100 {
            let result = normalize(MINIMAL_CONTRACT).unwrap();
            assert_eq!(first, result, "Determinism failure at iteration {}", i);
        }
    }

    #[test]
    fn test_determinism_100_iterations_complex() {
        let input = read_fixture("conformance/valid/all-primitive-types.icl");
        let first = normalize(&input).unwrap();

        for i in 0..100 {
            let result = normalize(&input).unwrap();
            assert_eq!(first, result, "Determinism failure at iteration {}", i);
        }
    }

    // ── Semantic preservation ──────────────────────────

    #[test]
    fn test_semantic_preservation() {
        // parse(normalize(x)) must preserve all semantic content of parse(x)
        let original = crate::parser::parse(MINIMAL_CONTRACT).unwrap();
        let normalized_text = normalize(MINIMAL_CONTRACT).unwrap();
        let normalized = crate::parser::parse(&normalized_text).unwrap();

        // Identity content preserved
        assert_eq!(
            original.identity.stable_id.value,
            normalized.identity.stable_id.value
        );
        assert_eq!(
            original.identity.version.value,
            normalized.identity.version.value
        );
        assert_eq!(
            original.identity.owner.value,
            normalized.identity.owner.value
        );

        // PurposeStatement preserved
        assert_eq!(
            original.purpose_statement.narrative.value,
            normalized.purpose_statement.narrative.value
        );
        assert_eq!(
            original.purpose_statement.confidence_level.value,
            normalized.purpose_statement.confidence_level.value
        );

        // State fields preserved (count)
        assert_eq!(
            original.data_semantics.state.len(),
            normalized.data_semantics.state.len()
        );

        // Operations preserved
        assert_eq!(
            original.behavioral_semantics.operations.len(),
            normalized.behavioral_semantics.operations.len()
        );

        // ExecutionConstraints preserved
        assert_eq!(
            original.execution_constraints.sandbox_mode.value,
            normalized.execution_constraints.sandbox_mode.value
        );
    }

    #[test]
    fn test_semantic_preservation_complex() {
        let input = read_fixture("conformance/valid/multiple-operations.icl");
        let original = crate::parser::parse(&input).unwrap();
        let normalized_text = normalize(&input).unwrap();
        let normalized = crate::parser::parse(&normalized_text).unwrap();

        assert_eq!(
            original.behavioral_semantics.operations.len(),
            normalized.behavioral_semantics.operations.len()
        );

        // All operation names preserved (may be reordered)
        let mut orig_names: Vec<_> = original
            .behavioral_semantics
            .operations
            .iter()
            .map(|o| o.name.value.clone())
            .collect();
        let mut norm_names: Vec<_> = normalized
            .behavioral_semantics
            .operations
            .iter()
            .map(|o| o.name.value.clone())
            .collect();
        orig_names.sort();
        norm_names.sort();
        assert_eq!(orig_names, norm_names);
    }

    // ── Conformance fixtures ───────────────────────────

    #[test]
    fn test_normalize_conformance_valid_minimal() {
        let input = read_fixture("conformance/valid/minimal-contract.icl");
        let normalized = normalize(&input).unwrap();
        let reparsed = crate::parser::parse(&normalized);
        assert!(
            reparsed.is_ok(),
            "Normalized valid/minimal-contract.icl doesn't reparse"
        );
    }

    #[test]
    fn test_normalize_conformance_valid_all_types() {
        let input = read_fixture("conformance/valid/all-primitive-types.icl");
        let normalized = normalize(&input).unwrap();
        let reparsed = crate::parser::parse(&normalized);
        assert!(
            reparsed.is_ok(),
            "Normalized valid/all-primitive-types.icl doesn't reparse"
        );
    }

    #[test]
    fn test_normalize_conformance_valid_composite() {
        let input = read_fixture("conformance/valid/composite-types.icl");
        let normalized = normalize(&input).unwrap();
        let reparsed = crate::parser::parse(&normalized);
        assert!(
            reparsed.is_ok(),
            "Normalized valid/composite-types.icl doesn't reparse"
        );
    }

    #[test]
    fn test_normalize_conformance_valid_operations() {
        let input = read_fixture("conformance/valid/multiple-operations.icl");
        let normalized = normalize(&input).unwrap();
        let reparsed = crate::parser::parse(&normalized);
        assert!(
            reparsed.is_ok(),
            "Normalized valid/multiple-operations.icl doesn't reparse"
        );
    }

    #[test]
    fn test_normalize_conformance_valid_extensions() {
        let input = read_fixture("conformance/valid/with-extensions.icl");
        let normalized = normalize(&input).unwrap();
        let reparsed = crate::parser::parse(&normalized);
        assert!(
            reparsed.is_ok(),
            "Normalized valid/with-extensions.icl doesn't reparse"
        );
    }
}
