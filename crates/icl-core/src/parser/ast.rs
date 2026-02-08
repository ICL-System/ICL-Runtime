//! ICL AST Types — Abstract Syntax Tree node definitions
//!
//! These types represent the parsed structure of an ICL contract.
//! They map directly to the BNF grammar in CORE-SPECIFICATION.md.
//!
//! All AST types are immutable after construction and derive:
//! Debug, Clone, PartialEq, Eq, Serialize, Deserialize

// AST types will be defined here as the parser is implemented.
// They will mirror the Contract structs in lib.rs but represent
// the parsed syntax tree before semantic analysis.
//
// For now, the Contract struct in lib.rs serves as both the
// AST and the semantic model. They will be separated as the
// parser grows in complexity.
//
// TODO: Phase 1.2 — Define ContractNode, IdentityNode, etc.
// TODO: Phase 1.2 — Define TypeExpression enum
// TODO: Phase 1.2 — Define OperationNode
// TODO: Phase 1.2 — Implement Display for all nodes
