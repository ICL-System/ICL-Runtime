//! ICL AST Types — Abstract Syntax Tree node definitions
//!
//! These types represent the parsed structure of an ICL contract.
//! They map directly to the BNF grammar in CORE-SPECIFICATION.md.
//!
//! The AST is the raw parse tree with source positions (spans).
//! A separate lowering step converts AST → semantic `Contract` in lib.rs.
//!
//! All AST types derive: Debug, Clone, PartialEq

use super::tokenizer::Span;

// ── Top-Level ──────────────────────────────────────────────

/// Root AST node for an ICL contract definition
#[derive(Debug, Clone, PartialEq)]
pub struct ContractNode {
    pub identity: IdentityNode,
    pub purpose_statement: PurposeStatementNode,
    pub data_semantics: DataSemanticsNode,
    pub behavioral_semantics: BehavioralSemanticsNode,
    pub execution_constraints: ExecutionConstraintsNode,
    pub human_machine_contract: HumanMachineContractNode,
    pub extensions: Option<ExtensionsNode>,
    pub span: Span,
}

// ── Identity (§1.2) ───────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct IdentityNode {
    pub stable_id: SpannedValue<String>,
    pub version: SpannedValue<i64>,
    pub created_timestamp: SpannedValue<String>,
    pub owner: SpannedValue<String>,
    pub semantic_hash: SpannedValue<String>,
    pub span: Span,
}

// ── Purpose Statement (§1.3) ──────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct PurposeStatementNode {
    pub narrative: SpannedValue<String>,
    pub intent_source: SpannedValue<String>,
    pub confidence_level: SpannedValue<f64>,
    pub span: Span,
}

// ── Data Semantics (§1.4) ─────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DataSemanticsNode {
    pub state: Vec<StateFieldNode>,
    pub invariants: Vec<SpannedValue<String>>,
    pub span: Span,
}

/// A field in a state definition or parameter list
#[derive(Debug, Clone, PartialEq)]
pub struct StateFieldNode {
    pub name: SpannedValue<String>,
    pub type_expr: TypeExpression,
    pub default_value: Option<LiteralValue>,
    pub span: Span,
}

// ── Type Expressions ──────────────────────────────────────

/// Type expression matching BNF grammar
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    /// Primitive: Integer, Float, String, Boolean, ISO8601, UUID
    Primitive(PrimitiveType, Span),
    /// Array<T>
    Array(Box<TypeExpression>, Span),
    /// Map<K, V>
    Map(Box<TypeExpression>, Box<TypeExpression>, Span),
    /// Object { fields... }
    Object(Vec<StateFieldNode>, Span),
    /// Enum ["a", "b", "c"]
    Enum(Vec<SpannedValue<String>>, Span),
}

/// ICL primitive types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Integer,
    Float,
    String,
    Boolean,
    Iso8601,
    Uuid,
}

/// Literal values for defaults and inline data
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    String(String, Span),
    Integer(i64, Span),
    Float(f64, Span),
    Boolean(bool, Span),
    Array(Vec<LiteralValue>, Span),
}

// ── Behavioral Semantics (§1.5) ───────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct BehavioralSemanticsNode {
    pub operations: Vec<OperationNode>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationNode {
    pub name: SpannedValue<String>,
    pub precondition: SpannedValue<String>,
    pub parameters: Vec<StateFieldNode>,
    pub postcondition: SpannedValue<String>,
    pub side_effects: Vec<SpannedValue<String>>,
    pub idempotence: SpannedValue<String>,
    pub span: Span,
}

// ── Execution Constraints (§1.6) ──────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionConstraintsNode {
    pub trigger_types: Vec<SpannedValue<String>>,
    pub resource_limits: ResourceLimitsNode,
    pub external_permissions: Vec<SpannedValue<String>>,
    pub sandbox_mode: SpannedValue<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceLimitsNode {
    pub max_memory_bytes: SpannedValue<i64>,
    pub computation_timeout_ms: SpannedValue<i64>,
    pub max_state_size_bytes: SpannedValue<i64>,
    pub span: Span,
}

// ── Human-Machine Contract (§1.7) ─────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct HumanMachineContractNode {
    pub system_commitments: Vec<SpannedValue<String>>,
    pub system_refusals: Vec<SpannedValue<String>>,
    pub user_obligations: Vec<SpannedValue<String>>,
    pub span: Span,
}

// ── Extensions (§5) ───────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionsNode {
    pub systems: Vec<SystemExtensionNode>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemExtensionNode {
    pub name: SpannedValue<String>,
    pub fields: Vec<CustomFieldNode>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFieldNode {
    pub name: SpannedValue<String>,
    pub value: LiteralValue,
    pub span: Span,
}

// ── Spanned Value (generic wrapper) ───────────────────────

/// A value annotated with its source span
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedValue<T> {
    pub value: T,
    pub span: Span,
}

impl<T> SpannedValue<T> {
    pub fn new(value: T, span: Span) -> Self {
        SpannedValue { value, span }
    }
}

// ── Display implementations ───────────────────────────────

impl std::fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PrimitiveType::Integer => write!(f, "Integer"),
            PrimitiveType::Float => write!(f, "Float"),
            PrimitiveType::String => write!(f, "String"),
            PrimitiveType::Boolean => write!(f, "Boolean"),
            PrimitiveType::Iso8601 => write!(f, "ISO8601"),
            PrimitiveType::Uuid => write!(f, "UUID"),
        }
    }
}

impl std::fmt::Display for TypeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeExpression::Primitive(p, _) => write!(f, "{}", p),
            TypeExpression::Array(inner, _) => write!(f, "Array<{}>", inner),
            TypeExpression::Map(k, v, _) => write!(f, "Map<{}, {}>", k, v),
            TypeExpression::Object(fields, _) => {
                write!(f, "Object {{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", field.name.value, field.type_expr)?;
                }
                write!(f, " }}")
            }
            TypeExpression::Enum(variants, _) => {
                write!(f, "Enum [")?;
                for (i, v) in variants.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\"", v.value)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LiteralValue::String(s, _) => write!(f, "\"{}\"", s),
            LiteralValue::Integer(n, _) => write!(f, "{}", n),
            LiteralValue::Float(n, _) => write!(f, "{}", n),
            LiteralValue::Boolean(b, _) => write!(f, "{}", b),            LiteralValue::Array(items, _) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }        }
    }
}

impl std::fmt::Display for ContractNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Contract(id={}, v={})", self.identity.stable_id.value, self.identity.version.value)
    }
}

// ── Span accessors (convenience) ──────────────────────────

impl TypeExpression {
    pub fn span(&self) -> &Span {
        match self {
            TypeExpression::Primitive(_, s) => s,
            TypeExpression::Array(_, s) => s,
            TypeExpression::Map(_, _, s) => s,
            TypeExpression::Object(_, s) => s,
            TypeExpression::Enum(_, s) => s,
        }
    }
}

impl LiteralValue {
    pub fn span(&self) -> &Span {
        match self {
            LiteralValue::String(_, s) => s,
            LiteralValue::Integer(_, s) => s,
            LiteralValue::Float(_, s) => s,
            LiteralValue::Boolean(_, s) => s,
            LiteralValue::Array(_, s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_type_display() {
        assert_eq!(PrimitiveType::Integer.to_string(), "Integer");
        assert_eq!(PrimitiveType::Float.to_string(), "Float");
        assert_eq!(PrimitiveType::String.to_string(), "String");
        assert_eq!(PrimitiveType::Boolean.to_string(), "Boolean");
        assert_eq!(PrimitiveType::Iso8601.to_string(), "ISO8601");
        assert_eq!(PrimitiveType::Uuid.to_string(), "UUID");
    }

    #[test]
    fn test_type_expression_display() {
        let span = Span { line: 1, column: 1, offset: 0 };

        let int_ty = TypeExpression::Primitive(PrimitiveType::Integer, span.clone());
        assert_eq!(int_ty.to_string(), "Integer");

        let arr_ty = TypeExpression::Array(
            Box::new(TypeExpression::Primitive(PrimitiveType::String, span.clone())),
            span.clone(),
        );
        assert_eq!(arr_ty.to_string(), "Array<String>");

        let map_ty = TypeExpression::Map(
            Box::new(TypeExpression::Primitive(PrimitiveType::String, span.clone())),
            Box::new(TypeExpression::Primitive(PrimitiveType::Integer, span.clone())),
            span.clone(),
        );
        assert_eq!(map_ty.to_string(), "Map<String, Integer>");
    }

    #[test]
    fn test_enum_display() {
        let span = Span { line: 1, column: 1, offset: 0 };
        let enum_ty = TypeExpression::Enum(
            vec![
                SpannedValue::new("active".to_string(), span.clone()),
                SpannedValue::new("inactive".to_string(), span.clone()),
            ],
            span.clone(),
        );
        assert_eq!(enum_ty.to_string(), r#"Enum ["active", "inactive"]"#);
    }

    #[test]
    fn test_literal_display() {
        let span = Span { line: 1, column: 1, offset: 0 };
        assert_eq!(LiteralValue::String("hello".to_string(), span.clone()).to_string(), "\"hello\"");
        assert_eq!(LiteralValue::Integer(42, span.clone()).to_string(), "42");
        assert_eq!(LiteralValue::Float(3.14, span.clone()).to_string(), "3.14");
        assert_eq!(LiteralValue::Boolean(true, span.clone()).to_string(), "true");
    }

    #[test]
    fn test_spanned_value() {
        let span = Span { line: 5, column: 10, offset: 50 };
        let sv = SpannedValue::new("test".to_string(), span.clone());
        assert_eq!(sv.value, "test");
        assert_eq!(sv.span, span);
    }

    #[test]
    fn test_type_expression_span() {
        let span = Span { line: 3, column: 7, offset: 30 };
        let ty = TypeExpression::Primitive(PrimitiveType::Boolean, span.clone());
        assert_eq!(ty.span(), &span);
    }

    #[test]
    fn test_object_display() {
        let span = Span { line: 1, column: 1, offset: 0 };
        let obj = TypeExpression::Object(
            vec![
                StateFieldNode {
                    name: SpannedValue::new("x".to_string(), span.clone()),
                    type_expr: TypeExpression::Primitive(PrimitiveType::Integer, span.clone()),
                    default_value: None,
                    span: span.clone(),
                },
                StateFieldNode {
                    name: SpannedValue::new("y".to_string(), span.clone()),
                    type_expr: TypeExpression::Primitive(PrimitiveType::Float, span.clone()),
                    default_value: None,
                    span: span.clone(),
                },
            ],
            span.clone(),
        );
        assert_eq!(obj.to_string(), "Object { x: Integer, y: Float }");
    }

    #[test]
    fn test_nested_type_display() {
        let span = Span { line: 1, column: 1, offset: 0 };
        // Array<Map<String, Integer>>
        let inner = TypeExpression::Map(
            Box::new(TypeExpression::Primitive(PrimitiveType::String, span.clone())),
            Box::new(TypeExpression::Primitive(PrimitiveType::Integer, span.clone())),
            span.clone(),
        );
        let outer = TypeExpression::Array(Box::new(inner), span.clone());
        assert_eq!(outer.to_string(), "Array<Map<String, Integer>>");
    }
}
