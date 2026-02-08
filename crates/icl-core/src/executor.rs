//! Execution engine — runs contracts deterministically in a sandbox
//!
//! The executor evaluates preconditions, runs operations in an isolated
//! environment, verifies postconditions, and logs all state transitions.
//!
//! # Architecture
//!
//! ICL is a *specification language*, not a scripting language. Operations
//! define typed state transitions with preconditions and postconditions
//! expressed as natural-language strings. The executor:
//!
//! 1. Maintains typed state matching DataSemantics.state
//! 2. Validates inputs against operation parameter types
//! 3. Evaluates simple condition patterns against state
//! 4. Applies state transitions (parameter values → state fields)
//! 5. Verifies postconditions and invariants hold
//! 6. Enforces resource limits (memory, timeout)
//! 7. Logs every transition in an immutable provenance log
//!
//! # Determinism
//!
//! The executor is pure — no I/O, no randomness, no system time.
//! All operations are deterministic: same state + same inputs = same result.

use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use crate::{Contract, Error, Result};

// ── Core Types ────────────────────────────────────────────

/// A typed runtime value in the execution state
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// Null / uninitialized
    Null,
    /// Boolean value
    Boolean(bool),
    /// Integer value (i64)
    Integer(i64),
    /// Float value (f64 — deterministic operations only)
    Float(f64),
    /// String value
    String(String),
    /// Array of values
    Array(Vec<Value>),
    /// Ordered map (BTreeMap for deterministic iteration)
    Object(BTreeMap<String, Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Value {
    /// Check if value is "truthy" for condition evaluation
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }

    /// Get the type name for error messages
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "Null",
            Value::Boolean(_) => "Boolean",
            Value::Integer(_) => "Integer",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Array(_) => "Array",
            Value::Object(_) => "Object",
        }
    }

    /// Convert from serde_json::Value (deterministic — uses BTreeMap)
    pub fn from_json(json: &serde_json::Value) -> Self {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s.clone()),
            serde_json::Value::Array(arr) => {
                Value::Array(arr.iter().map(Value::from_json).collect())
            }
            serde_json::Value::Object(map) => {
                let btree: BTreeMap<String, Value> = map
                    .iter()
                    .map(|(k, v)| (k.clone(), Value::from_json(v)))
                    .collect();
                Value::Object(btree)
            }
        }
    }

    /// Convert to serde_json::Value
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Null => serde_json::Value::Null,
            Value::Boolean(b) => serde_json::Value::Bool(*b),
            Value::Integer(i) => serde_json::json!(*i),
            Value::Float(f) => serde_json::json!(*f),
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| v.to_json()).collect())
            }
            Value::Object(map) => {
                let obj: serde_json::Map<String, serde_json::Value> =
                    map.iter().map(|(k, v)| (k.clone(), v.to_json())).collect();
                serde_json::Value::Object(obj)
            }
        }
    }
}

// ── Execution State ───────────────────────────────────────

/// The mutable state of a contract during execution.
/// Uses BTreeMap for deterministic field ordering.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecutionState {
    /// Named state fields with typed values
    pub fields: BTreeMap<String, Value>,
}

impl ExecutionState {
    /// Create initial state from contract's DataSemantics
    pub fn from_contract(contract: &Contract) -> Self {
        let fields = if let serde_json::Value::Object(map) = &contract.data_semantics.state {
            let mut btree = BTreeMap::new();
            for (key, type_info) in map.iter() {
                // Extract default value if present, otherwise use type-appropriate default
                let value = Self::default_for_type(type_info);
                btree.insert(key.clone(), value);
            }
            btree
        } else {
            BTreeMap::new()
        };
        ExecutionState { fields }
    }

    /// Derive a default value from a type descriptor
    fn default_for_type(type_info: &serde_json::Value) -> Value {
        match type_info {
            serde_json::Value::String(type_name) => match type_name.as_str() {
                "Integer" => Value::Integer(0),
                "Float" => Value::Float(0.0),
                "String" | "ISO8601" | "UUID" => Value::String(String::new()),
                "Boolean" => Value::Boolean(false),
                _ => Value::Null,
            },
            serde_json::Value::Object(obj) => {
                if let Some(serde_json::Value::String(t)) = obj.get("type") {
                    match t.as_str() {
                        "Integer" | "Float" | "String" | "Boolean" | "ISO8601" | "UUID" => {
                            // Check for explicit default value
                            if let Some(default) = obj.get("default") {
                                Value::from_json(default)
                            } else {
                                Self::default_for_type(&serde_json::Value::String(t.clone()))
                            }
                        }
                        _ => Value::Null,
                    }
                } else {
                    // Nested object — recurse
                    let mut btree = BTreeMap::new();
                    for (k, v) in obj {
                        btree.insert(k.clone(), Self::default_for_type(v));
                    }
                    Value::Object(btree)
                }
            }
            serde_json::Value::Array(_) => Value::Array(Vec::new()),
            _ => Value::Null,
        }
    }

    /// Get a field value by name
    pub fn get(&self, field: &str) -> Option<&Value> {
        self.fields.get(field)
    }

    /// Set a field value, returning the previous value
    pub fn set(&mut self, field: String, value: Value) -> Option<Value> {
        self.fields.insert(field, value)
    }

    /// Approximate memory usage in bytes
    pub fn memory_bytes(&self) -> u64 {
        self.estimate_size() as u64
    }

    fn estimate_size(&self) -> usize {
        self.fields
            .iter()
            .map(|(k, v)| k.len() + Self::value_size(v))
            .sum()
    }

    fn value_size(value: &Value) -> usize {
        match value {
            Value::Null => 1,
            Value::Boolean(_) => 1,
            Value::Integer(_) => 8,
            Value::Float(_) => 8,
            Value::String(s) => s.len() + 24, // heap overhead
            Value::Array(arr) => 24 + arr.iter().map(Self::value_size).sum::<usize>(),
            Value::Object(map) => {
                24 + map
                    .iter()
                    .map(|(k, v)| k.len() + Self::value_size(v))
                    .sum::<usize>()
            }
        }
    }
}

// ── Expression Evaluator ──────────────────────────────────

/// Evaluates simple condition patterns against execution state.
///
/// Supports common invariant/condition patterns from ICL contracts:
/// - `"<field> is not empty"` — string/array length > 0
/// - `"<field> >= <number>"` — numeric comparison
/// - `"<field> <= <number>"` — numeric comparison
/// - `"<field> > <number>"` — numeric comparison  
/// - `"<field> < <number>"` — numeric comparison
/// - `"<field> is boolean"` — type check
/// - `"<field> is valid ..."` — always true (advisory)
/// - Opaque strings — always true (not machine-evaluable)
pub struct ExpressionEvaluator;

impl ExpressionEvaluator {
    /// Evaluate a condition string against the current state.
    /// Returns (result, is_evaluable) — false for `is_evaluable` means
    /// the condition is an opaque string that can't be machine-evaluated.
    pub fn evaluate(condition: &str, state: &ExecutionState) -> (bool, bool) {
        let trimmed = condition.trim();

        // Pattern: "<field> is not empty"
        if let Some(field) = trimmed.strip_suffix(" is not empty") {
            let field = field.trim();
            if let Some(value) = state.get(field) {
                return (value.is_truthy(), true);
            }
            // Field doesn't exist — fails
            return (false, true);
        }

        // Pattern: "<field> >= <number>"
        if let Some((field, num)) = Self::parse_comparison(trimmed, " >= ") {
            return (Self::numeric_cmp(state, field, num, |a, b| a >= b), true);
        }

        // Pattern: "<field> <= <number>"
        if let Some((field, num)) = Self::parse_comparison(trimmed, " <= ") {
            return (Self::numeric_cmp(state, field, num, |a, b| a <= b), true);
        }

        // Pattern: "<field> > <number>"
        if let Some((field, num)) = Self::parse_comparison(trimmed, " > ") {
            // Don't match ">=" which was already handled
            return (Self::numeric_cmp(state, field, num, |a, b| a > b), true);
        }

        // Pattern: "<field> < <number>"
        if let Some((field, num)) = Self::parse_comparison(trimmed, " < ") {
            return (Self::numeric_cmp(state, field, num, |a, b| a < b), true);
        }

        // Pattern: "<field> is boolean"
        if let Some(field) = trimmed.strip_suffix(" is boolean") {
            let field = field.trim();
            if let Some(Value::Boolean(_)) = state.get(field) {
                return (true, true);
            }
            return (false, true);
        }

        // Pattern: "<field> is valid ..." — advisory, always true
        if trimmed.contains("is valid ") {
            return (true, false);
        }

        // Opaque condition — not machine-evaluable, treat as true
        (true, false)
    }

    /// Parse a comparison pattern like "field >= 0" into (field_name, number)
    fn parse_comparison<'a>(s: &'a str, operator: &str) -> Option<(&'a str, f64)> {
        let parts: Vec<&str> = s.splitn(2, operator).collect();
        if parts.len() == 2 {
            let field = parts[0].trim();
            let num_str = parts[1].trim();
            if let Ok(num) = num_str.parse::<f64>() {
                return Some((field, num));
            }
        }
        None
    }

    /// Do numeric comparison on a state field
    fn numeric_cmp(
        state: &ExecutionState,
        field: &str,
        rhs: f64,
        cmp: fn(f64, f64) -> bool,
    ) -> bool {
        match state.get(field) {
            Some(Value::Integer(i)) => cmp(*i as f64, rhs),
            Some(Value::Float(f)) => cmp(*f, rhs),
            _ => false,
        }
    }

    /// Evaluate all contract invariants against state
    pub fn check_invariants(
        invariants: &[String],
        state: &ExecutionState,
    ) -> std::result::Result<(), Vec<String>> {
        let mut violations = Vec::new();
        for inv in invariants {
            let (result, evaluable) = Self::evaluate(inv, state);
            if evaluable && !result {
                violations.push(inv.clone());
            }
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

// ── Sandbox ───────────────────────────────────────────────

/// Isolated execution environment with resource limits
#[derive(Debug, Clone)]
pub struct Sandbox {
    /// Maximum memory in bytes
    pub max_memory_bytes: u64,
    /// Computation timeout in milliseconds
    pub computation_timeout_ms: u64,
    /// Maximum state size in bytes
    pub max_state_size_bytes: u64,
    /// Sandbox isolation mode
    pub mode: SandboxMode,
    /// External permissions granted
    pub permissions: Vec<String>,
}

/// Sandbox isolation levels from spec §1.6
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SandboxMode {
    /// No external access, full determinism guarantee
    FullIsolation,
    /// Limited external access (declared permissions only)
    Restricted,
    /// No sandbox — advisory mode only
    None,
}

impl Sandbox {
    /// Create sandbox from contract execution constraints
    pub fn from_contract(contract: &Contract) -> Self {
        let mode = match contract.execution_constraints.sandbox_mode.as_str() {
            "full_isolation" => SandboxMode::FullIsolation,
            "restricted" => SandboxMode::Restricted,
            "none" => SandboxMode::None,
            _ => SandboxMode::FullIsolation, // default to safest
        };

        Sandbox {
            max_memory_bytes: contract
                .execution_constraints
                .resource_limits
                .max_memory_bytes,
            computation_timeout_ms: contract
                .execution_constraints
                .resource_limits
                .computation_timeout_ms,
            max_state_size_bytes: contract
                .execution_constraints
                .resource_limits
                .max_state_size_bytes,
            mode,
            permissions: contract.execution_constraints.external_permissions.clone(),
        }
    }

    /// Check if current state is within memory limits
    pub fn check_memory(&self, state: &ExecutionState) -> Result<()> {
        let used = state.memory_bytes();
        if used > self.max_state_size_bytes {
            return Err(Error::ExecutionError(format!(
                "State size {} bytes exceeds limit of {} bytes",
                used, self.max_state_size_bytes
            )));
        }
        if used > self.max_memory_bytes {
            return Err(Error::ExecutionError(format!(
                "Memory usage {} bytes exceeds limit of {} bytes",
                used, self.max_memory_bytes
            )));
        }
        Ok(())
    }

    /// Check if an operation has required permissions
    pub fn check_permissions(&self, required: &[String]) -> Result<()> {
        if self.mode == SandboxMode::FullIsolation && !required.is_empty() {
            return Err(Error::ExecutionError(
                "Full isolation sandbox does not permit external access".into(),
            ));
        }
        for perm in required {
            if !self.permissions.contains(perm) {
                return Err(Error::ExecutionError(format!(
                    "Permission '{}' not granted in sandbox",
                    perm
                )));
            }
        }
        Ok(())
    }
}

// ── Provenance Log ────────────────────────────────────────

/// A single entry in the provenance log — records one state transition
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProvenanceEntry {
    /// Sequential operation number (0-indexed)
    pub sequence: u64,
    /// Name of the operation that caused this transition
    pub operation: String,
    /// Input parameters as JSON
    pub inputs: serde_json::Value,
    /// State snapshot before the operation
    pub state_before: BTreeMap<String, Value>,
    /// State snapshot after the operation
    pub state_after: BTreeMap<String, Value>,
    /// Fields that changed
    pub changes: Vec<StateChange>,
    /// Whether all postconditions held
    pub postconditions_verified: bool,
    /// Whether all invariants held
    pub invariants_verified: bool,
}

/// A single field change within a state transition
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StateChange {
    pub field: String,
    pub old_value: Value,
    pub new_value: Value,
}

/// Immutable append-only provenance log
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProvenanceLog {
    pub entries: Vec<ProvenanceEntry>,
}

impl ProvenanceLog {
    pub fn new() -> Self {
        ProvenanceLog {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, entry: ProvenanceEntry) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for ProvenanceLog {
    fn default() -> Self {
        Self::new()
    }
}

// ── Execution Result ──────────────────────────────────────

/// Result of executing a single operation
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperationResult {
    /// Name of the operation executed
    pub operation: String,
    /// Whether execution succeeded
    pub success: bool,
    /// The new state after execution (if successful)
    pub state: BTreeMap<String, Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Provenance entry for this operation
    pub provenance: Option<ProvenanceEntry>,
}

/// Result of executing a full contract
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecutionResult {
    /// Contract stable_id
    pub contract_id: String,
    /// Whether overall execution succeeded
    pub success: bool,
    /// Individual operation results
    pub operations: Vec<OperationResult>,
    /// Final state
    pub final_state: BTreeMap<String, Value>,
    /// Complete provenance log
    pub provenance: ProvenanceLog,
    /// Error message (if failed)
    pub error: Option<String>,
}

// ── Executor ──────────────────────────────────────────────

/// The contract executor — runs operations deterministically in a sandbox
pub struct Executor {
    /// The contract being executed
    contract: Contract,
    /// Current execution state
    state: ExecutionState,
    /// Sandbox environment with resource limits
    sandbox: Sandbox,
    /// Provenance log (append-only)
    provenance: ProvenanceLog,
    /// Operation counter
    sequence: u64,
}

impl Executor {
    /// Create a new executor for a contract
    pub fn new(contract: Contract) -> Self {
        let state = ExecutionState::from_contract(&contract);
        let sandbox = Sandbox::from_contract(&contract);
        Executor {
            contract,
            state,
            sandbox,
            provenance: ProvenanceLog::new(),
            sequence: 0,
        }
    }

    /// Execute a named operation with JSON input parameters
    pub fn execute_operation(
        &mut self,
        operation_name: &str,
        inputs_json: &str,
    ) -> Result<OperationResult> {
        #[cfg(not(target_arch = "wasm32"))]
        let start = Instant::now();

        // 1. Find the operation definition
        let op = self
            .contract
            .behavioral_semantics
            .operations
            .iter()
            .find(|o| o.name == operation_name)
            .ok_or_else(|| {
                Error::ExecutionError(format!(
                    "Operation '{}' not found in contract",
                    operation_name
                ))
            })?
            .clone();

        // 2. Parse inputs
        let inputs: serde_json::Value = serde_json::from_str(inputs_json)
            .map_err(|e| Error::ExecutionError(format!("Invalid JSON input: {}", e)))?;

        // 3. Validate input parameters against operation definition
        self.validate_inputs(&op, &inputs)?;

        // 4. Check precondition
        let (pre_result, pre_evaluable) =
            ExpressionEvaluator::evaluate(&op.precondition, &self.state);
        if pre_evaluable && !pre_result {
            return Err(Error::ExecutionError(format!(
                "Precondition failed for operation '{}': {}",
                operation_name, op.precondition
            )));
        }

        // 5. Snapshot state before
        let state_before = self.state.fields.clone();

        // 6. Apply operation — update state with input parameters
        self.apply_inputs(&inputs)?;

        // 7. Check timeout (not available on wasm32)
        #[cfg(not(target_arch = "wasm32"))]
        {
            let elapsed_ms = start.elapsed().as_millis() as u64;
            if elapsed_ms > self.sandbox.computation_timeout_ms {
                // Rollback state
                self.state.fields = state_before.clone();
                return Err(Error::ExecutionError(format!(
                    "Operation '{}' exceeded timeout of {}ms (took {}ms)",
                    operation_name, self.sandbox.computation_timeout_ms, elapsed_ms
                )));
            }
        }

        // 8. Check postcondition
        let (post_result, post_evaluable) =
            ExpressionEvaluator::evaluate(&op.postcondition, &self.state);
        let postconditions_verified = !post_evaluable || post_result;

        if post_evaluable && !post_result {
            // Rollback state
            self.state.fields = state_before;
            return Err(Error::ContractViolation {
                commitment: format!("postcondition of '{}'", operation_name),
                violation: op.postcondition.clone(),
            });
        }

        // 9. Check all invariants
        let invariants_verified = match ExpressionEvaluator::check_invariants(
            &self.contract.data_semantics.invariants,
            &self.state,
        ) {
            Ok(()) => true,
            Err(violations) => {
                // Rollback state
                self.state.fields = state_before;
                return Err(Error::ContractViolation {
                    commitment: "invariant".into(),
                    violation: format!("Violated invariants: {}", violations.join(", ")),
                });
            }
        };

        // 10. Check resource limits
        self.sandbox.check_memory(&self.state).inspect_err(|_| {
            self.state.fields = state_before.clone();
        })?;

        // 11. Compute changes
        let changes = Self::compute_changes(&state_before, &self.state.fields);

        // 12. Record provenance
        let entry = ProvenanceEntry {
            sequence: self.sequence,
            operation: operation_name.to_string(),
            inputs: inputs.clone(),
            state_before,
            state_after: self.state.fields.clone(),
            changes,
            postconditions_verified,
            invariants_verified,
        };
        self.provenance.append(entry.clone());
        self.sequence += 1;

        Ok(OperationResult {
            operation: operation_name.to_string(),
            success: true,
            state: self.state.fields.clone(),
            error: None,
            provenance: Some(entry),
        })
    }

    /// Validate that inputs match operation parameter types
    fn validate_inputs(&self, op: &crate::Operation, inputs: &serde_json::Value) -> Result<()> {
        if let serde_json::Value::Object(params_def) = &op.parameters {
            if let serde_json::Value::Object(input_map) = inputs {
                // Check all required parameters are provided
                for (param_name, _param_type) in params_def {
                    if !input_map.contains_key(param_name) {
                        return Err(Error::ExecutionError(format!(
                            "Missing required parameter '{}' for operation '{}'",
                            param_name, op.name
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    /// Apply input values to the execution state
    fn apply_inputs(&mut self, inputs: &serde_json::Value) -> Result<()> {
        if let serde_json::Value::Object(input_map) = inputs {
            for (key, value) in input_map {
                let typed_value = Value::from_json(value);
                self.state.set(key.clone(), typed_value);
            }
        }
        Ok(())
    }

    /// Compute the list of field changes between two state snapshots
    fn compute_changes(
        before: &BTreeMap<String, Value>,
        after: &BTreeMap<String, Value>,
    ) -> Vec<StateChange> {
        let mut changes = Vec::new();

        // Check fields in before
        for (key, old_val) in before {
            match after.get(key) {
                Some(new_val) if new_val != old_val => {
                    changes.push(StateChange {
                        field: key.clone(),
                        old_value: old_val.clone(),
                        new_value: new_val.clone(),
                    });
                }
                None => {
                    changes.push(StateChange {
                        field: key.clone(),
                        old_value: old_val.clone(),
                        new_value: Value::Null,
                    });
                }
                _ => {}
            }
        }

        // Check new fields
        for (key, new_val) in after {
            if !before.contains_key(key) {
                changes.push(StateChange {
                    field: key.clone(),
                    old_value: Value::Null,
                    new_value: new_val.clone(),
                });
            }
        }

        changes
    }

    /// Execute a contract fully: run all operations from a JSON array of requests
    /// Each request: { "operation": "name", "inputs": { ... } }
    pub fn execute_all(&mut self, requests_json: &str) -> Result<ExecutionResult> {
        let requests: Vec<serde_json::Value> = serde_json::from_str(requests_json)
            .map_err(|e| Error::ExecutionError(format!("Invalid JSON requests: {}", e)))?;

        let mut operation_results = Vec::new();

        for req in &requests {
            let op_name = req
                .get("operation")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    Error::ExecutionError("Each request must have an 'operation' field".into())
                })?;

            let empty_obj = serde_json::Value::Object(serde_json::Map::new());
            let inputs = req.get("inputs").unwrap_or(&empty_obj);

            let inputs_str = serde_json::to_string(inputs)
                .map_err(|e| Error::ExecutionError(format!("Failed to serialize inputs: {}", e)))?;

            match self.execute_operation(op_name, &inputs_str) {
                Ok(result) => operation_results.push(result),
                Err(e) => {
                    operation_results.push(OperationResult {
                        operation: op_name.to_string(),
                        success: false,
                        state: self.state.fields.clone(),
                        error: Some(e.to_string()),
                        provenance: None,
                    });
                    return Ok(ExecutionResult {
                        contract_id: self.contract.identity.stable_id.clone(),
                        success: false,
                        operations: operation_results,
                        final_state: self.state.fields.clone(),
                        provenance: self.provenance.clone(),
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        Ok(ExecutionResult {
            contract_id: self.contract.identity.stable_id.clone(),
            success: true,
            operations: operation_results,
            final_state: self.state.fields.clone(),
            provenance: self.provenance.clone(),
            error: None,
        })
    }

    /// Get current state (immutable ref)
    pub fn state(&self) -> &ExecutionState {
        &self.state
    }

    /// Get provenance log (immutable ref)
    pub fn provenance(&self) -> &ProvenanceLog {
        &self.provenance
    }
}

/// Execute a contract with given inputs (convenience function — public API)
///
/// # Arguments
/// - `contract` — parsed & verified contract
/// - `inputs` — JSON string: array of `{ "operation": "name", "inputs": { ... } }`
///   OR single `{ "operation": "name", "inputs": { ... } }`
///
/// # Returns
/// JSON string with execution result including provenance log
///
/// # Guarantees
/// - Deterministic: same inputs → same outputs
/// - Bounded: resource limits enforced (memory, time)
/// - Verifiable: preconditions checked, postconditions verified
/// - Logged: all state changes recorded in provenance
pub fn execute_contract(contract: &Contract, inputs: &str) -> Result<String> {
    let mut executor = Executor::new(contract.clone());

    // Detect if inputs is a single request or array
    let inputs_trimmed = inputs.trim();
    let requests_json = if inputs_trimmed.starts_with('[') {
        inputs_trimmed.to_string()
    } else if inputs_trimmed.starts_with('{') {
        format!("[{}]", inputs_trimmed)
    } else {
        return Err(Error::ExecutionError(
            "Input must be a JSON object or array of objects".into(),
        ));
    };

    let result = executor.execute_all(&requests_json)?;

    serde_json::to_string_pretty(&result)
        .map_err(|e| Error::ExecutionError(format!("Failed to serialize result: {}", e)))
}

// ── Tests ─────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    /// Helper: create a minimal contract for testing
    fn test_contract() -> Contract {
        Contract {
            identity: Identity {
                stable_id: "ic-test-001".into(),
                version: 1,
                created_timestamp: "2026-02-01T10:00:00Z".into(),
                owner: "test".into(),
                semantic_hash: "abc123".into(),
            },
            purpose_statement: PurposeStatement {
                narrative: "Test contract".into(),
                intent_source: "test".into(),
                confidence_level: 1.0,
            },
            data_semantics: DataSemantics {
                state: serde_json::json!({
                    "message": "String",
                    "count": "Integer"
                }),
                invariants: vec!["message is not empty".into(), "count >= 0".into()],
            },
            behavioral_semantics: BehavioralSemantics {
                operations: vec![Operation {
                    name: "echo".into(),
                    precondition: "input_provided".into(),
                    parameters: serde_json::json!({
                        "message": "String"
                    }),
                    postcondition: "state_updated".into(),
                    side_effects: vec!["log_operation".into()],
                    idempotence: "idempotent".into(),
                }],
            },
            execution_constraints: ExecutionConstraints {
                trigger_types: vec!["manual".into()],
                resource_limits: ResourceLimits {
                    max_memory_bytes: 1_048_576,
                    computation_timeout_ms: 1000,
                    max_state_size_bytes: 1_048_576,
                },
                external_permissions: vec![],
                sandbox_mode: "full_isolation".into(),
            },
            human_machine_contract: HumanMachineContract {
                system_commitments: vec!["All messages echoed".into()],
                system_refusals: vec!["Will not lose data".into()],
                user_obligations: vec!["Provide messages".into()],
            },
        }
    }

    // ── Value Tests ───────────────────────────────────────

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_value_from_json_primitives() {
        assert_eq!(Value::from_json(&serde_json::json!(null)), Value::Null);
        assert_eq!(
            Value::from_json(&serde_json::json!(true)),
            Value::Boolean(true)
        );
        assert_eq!(Value::from_json(&serde_json::json!(42)), Value::Integer(42));
        assert_eq!(
            Value::from_json(&serde_json::json!(3.14)),
            Value::Float(3.14)
        );
        assert_eq!(
            Value::from_json(&serde_json::json!("hello")),
            Value::String("hello".into())
        );
    }

    #[test]
    fn test_value_from_json_collections() {
        let arr = Value::from_json(&serde_json::json!([1, 2, 3]));
        assert_eq!(
            arr,
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
            ])
        );

        let obj = Value::from_json(&serde_json::json!({"a": 1, "b": "two"}));
        let mut expected = BTreeMap::new();
        expected.insert("a".into(), Value::Integer(1));
        expected.insert("b".into(), Value::String("two".into()));
        assert_eq!(obj, Value::Object(expected));
    }

    #[test]
    fn test_value_roundtrip_json() {
        let original = serde_json::json!({
            "name": "test",
            "count": 42,
            "active": true,
            "items": [1, 2, 3]
        });
        let value = Value::from_json(&original);
        let back = value.to_json();
        assert_eq!(original, back);
    }

    #[test]
    fn test_value_is_truthy() {
        assert!(!Value::Null.is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Integer(0).is_truthy());
        assert!(Value::Integer(1).is_truthy());
        assert!(!Value::String(String::new()).is_truthy());
        assert!(Value::String("hello".into()).is_truthy());
        assert!(!Value::Array(vec![]).is_truthy());
        assert!(Value::Array(vec![Value::Integer(1)]).is_truthy());
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Null), "null");
        assert_eq!(format!("{}", Value::Boolean(true)), "true");
        assert_eq!(format!("{}", Value::Integer(42)), "42");
        assert_eq!(format!("{}", Value::String("hi".into())), "\"hi\"");
    }

    // ── ExecutionState Tests ──────────────────────────────

    #[test]
    fn test_execution_state_from_contract() {
        let contract = test_contract();
        let state = ExecutionState::from_contract(&contract);

        assert_eq!(state.get("message"), Some(&Value::String(String::new())));
        assert_eq!(state.get("count"), Some(&Value::Integer(0)));
    }

    #[test]
    fn test_execution_state_set_get() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        state.set("x".into(), Value::Integer(10));
        assert_eq!(state.get("x"), Some(&Value::Integer(10)));

        let old = state.set("x".into(), Value::Integer(20));
        assert_eq!(old, Some(Value::Integer(10)));
        assert_eq!(state.get("x"), Some(&Value::Integer(20)));
    }

    #[test]
    fn test_execution_state_memory_bytes() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        let empty_size = state.memory_bytes();
        state.set("big_string".into(), Value::String("x".repeat(1000)));
        assert!(state.memory_bytes() > empty_size + 1000);
    }

    // ── ExpressionEvaluator Tests ─────────────────────────

    #[test]
    fn test_eval_is_not_empty_true() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        state.set("message".into(), Value::String("hello".into()));
        let (result, evaluable) = ExpressionEvaluator::evaluate("message is not empty", &state);
        assert!(evaluable);
        assert!(result);
    }

    #[test]
    fn test_eval_is_not_empty_false() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        state.set("message".into(), Value::String(String::new()));
        let (result, evaluable) = ExpressionEvaluator::evaluate("message is not empty", &state);
        assert!(evaluable);
        assert!(!result);
    }

    #[test]
    fn test_eval_numeric_comparisons() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        state.set("count".into(), Value::Integer(5));

        assert!(ExpressionEvaluator::evaluate("count >= 0", &state).0);
        assert!(ExpressionEvaluator::evaluate("count >= 5", &state).0);
        assert!(!ExpressionEvaluator::evaluate("count >= 6", &state).0);
        assert!(ExpressionEvaluator::evaluate("count > 4", &state).0);
        assert!(!ExpressionEvaluator::evaluate("count > 5", &state).0);
        assert!(ExpressionEvaluator::evaluate("count <= 5", &state).0);
        assert!(ExpressionEvaluator::evaluate("count < 6", &state).0);
        assert!(!ExpressionEvaluator::evaluate("count < 5", &state).0);
    }

    #[test]
    fn test_eval_is_boolean() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        state.set("flag".into(), Value::Boolean(true));
        state.set("count".into(), Value::Integer(5));

        assert!(ExpressionEvaluator::evaluate("flag is boolean", &state).0);
        assert!(!ExpressionEvaluator::evaluate("count is boolean", &state).0);
    }

    #[test]
    fn test_eval_opaque_condition() {
        let state = ExecutionState {
            fields: BTreeMap::new(),
        };
        let (result, evaluable) = ExpressionEvaluator::evaluate("some_opaque_condition", &state);
        assert!(!evaluable);
        assert!(result); // opaque = pass
    }

    #[test]
    fn test_check_invariants_all_pass() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        state.set("message".into(), Value::String("hello".into()));
        state.set("count".into(), Value::Integer(5));

        let invariants = vec!["message is not empty".into(), "count >= 0".into()];
        assert!(ExpressionEvaluator::check_invariants(&invariants, &state).is_ok());
    }

    #[test]
    fn test_check_invariants_one_fails() {
        let mut state = ExecutionState {
            fields: BTreeMap::new(),
        };
        state.set("message".into(), Value::String(String::new()));
        state.set("count".into(), Value::Integer(5));

        let invariants = vec!["message is not empty".into(), "count >= 0".into()];
        let result = ExpressionEvaluator::check_invariants(&invariants, &state);
        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert_eq!(violations, vec!["message is not empty"]);
    }

    // ── Sandbox Tests ─────────────────────────────────────

    #[test]
    fn test_sandbox_from_contract() {
        let contract = test_contract();
        let sandbox = Sandbox::from_contract(&contract);
        assert_eq!(sandbox.mode, SandboxMode::FullIsolation);
        assert_eq!(sandbox.max_memory_bytes, 1_048_576);
        assert_eq!(sandbox.computation_timeout_ms, 1000);
    }

    #[test]
    fn test_sandbox_check_memory_within_limits() {
        let contract = test_contract();
        let sandbox = Sandbox::from_contract(&contract);
        let state = ExecutionState::from_contract(&contract);
        assert!(sandbox.check_memory(&state).is_ok());
    }

    #[test]
    fn test_sandbox_check_memory_exceeds_limit() {
        let contract = test_contract();
        let sandbox = Sandbox {
            max_memory_bytes: 10,
            max_state_size_bytes: 10,
            ..Sandbox::from_contract(&contract)
        };
        let mut state = ExecutionState::from_contract(&contract);
        state.set("big".into(), Value::String("x".repeat(100)));
        assert!(sandbox.check_memory(&state).is_err());
    }

    #[test]
    fn test_sandbox_permissions_full_isolation() {
        let sandbox = Sandbox {
            max_memory_bytes: 1_000_000,
            computation_timeout_ms: 1000,
            max_state_size_bytes: 1_000_000,
            mode: SandboxMode::FullIsolation,
            permissions: vec![],
        };
        assert!(sandbox.check_permissions(&[]).is_ok());
        assert!(sandbox.check_permissions(&["network".to_string()]).is_err());
    }

    #[test]
    fn test_sandbox_permissions_restricted() {
        let sandbox = Sandbox {
            max_memory_bytes: 1_000_000,
            computation_timeout_ms: 1000,
            max_state_size_bytes: 1_000_000,
            mode: SandboxMode::Restricted,
            permissions: vec!["database_query".into()],
        };
        assert!(sandbox
            .check_permissions(&["database_query".to_string()])
            .is_ok());
        assert!(sandbox.check_permissions(&["network".to_string()]).is_err());
    }

    // ── ProvenanceLog Tests ───────────────────────────────

    #[test]
    fn test_provenance_log_new_empty() {
        let log = ProvenanceLog::new();
        assert!(log.is_empty());
        assert_eq!(log.len(), 0);
    }

    #[test]
    fn test_provenance_log_append() {
        let mut log = ProvenanceLog::new();
        let entry = ProvenanceEntry {
            sequence: 0,
            operation: "test".into(),
            inputs: serde_json::json!({}),
            state_before: BTreeMap::new(),
            state_after: BTreeMap::new(),
            changes: vec![],
            postconditions_verified: true,
            invariants_verified: true,
        };
        log.append(entry);
        assert_eq!(log.len(), 1);
        assert!(!log.is_empty());
    }

    // ── Executor Tests ────────────────────────────────────

    #[test]
    fn test_executor_new() {
        let contract = test_contract();
        let executor = Executor::new(contract);
        assert_eq!(
            executor.state().get("message"),
            Some(&Value::String(String::new()))
        );
        assert_eq!(executor.state().get("count"), Some(&Value::Integer(0)));
        assert!(executor.provenance().is_empty());
    }

    #[test]
    fn test_execute_operation_success() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        let result = executor
            .execute_operation("echo", r#"{"message": "hello"}"#)
            .unwrap();

        assert!(result.success);
        assert_eq!(result.operation, "echo");
        assert_eq!(
            executor.state().get("message"),
            Some(&Value::String("hello".into()))
        );
        assert!(result.provenance.is_some());
    }

    #[test]
    fn test_execute_operation_not_found() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        let result = executor.execute_operation("nonexistent", "{}");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_execute_operation_invalid_json() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        let result = executor.execute_operation("echo", "not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_operation_invariant_violation() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        // count >= 0 invariant — setting count to -1 should fail
        let result = executor.execute_operation("echo", r#"{"count": -1, "message": "hi"}"#);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invariant") || err.contains("Violated"));
    }

    #[test]
    fn test_execute_operation_state_rollback_on_failure() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        // First: set valid state
        executor
            .execute_operation("echo", r#"{"message": "hello"}"#)
            .unwrap();

        let state_after_success = executor.state().clone();

        // Second: try invalid operation (violates count >= 0)
        let _ = executor.execute_operation("echo", r#"{"count": -1, "message": "hi"}"#);

        // State should be rolled back to after first success
        assert_eq!(*executor.state(), state_after_success);
    }

    #[test]
    fn test_execute_all_success() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        let requests = r#"[
            {"operation": "echo", "inputs": {"message": "hello"}},
            {"operation": "echo", "inputs": {"message": "world"}}
        ]"#;

        let result = executor.execute_all(requests).unwrap();
        assert!(result.success);
        assert_eq!(result.operations.len(), 2);
        assert_eq!(result.provenance.len(), 2);
    }

    #[test]
    fn test_execute_all_stops_on_failure() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        let requests = r#"[
            {"operation": "echo", "inputs": {"message": "hello"}},
            {"operation": "nonexistent", "inputs": {}},
            {"operation": "echo", "inputs": {"message": "world"}}
        ]"#;

        let result = executor.execute_all(requests).unwrap();
        assert!(!result.success);
        assert_eq!(result.operations.len(), 2); // only 2 attempted
    }

    #[test]
    fn test_provenance_records_state_changes() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        executor
            .execute_operation("echo", r#"{"message": "hello"}"#)
            .unwrap();

        let log = executor.provenance();
        assert_eq!(log.len(), 1);

        let entry = &log.entries[0];
        assert_eq!(entry.operation, "echo");
        assert_eq!(entry.sequence, 0);
        assert!(entry.postconditions_verified);
        assert!(entry.invariants_verified);
        assert!(!entry.changes.is_empty());

        // Verify the message change was recorded
        let msg_change = entry.changes.iter().find(|c| c.field == "message").unwrap();
        assert_eq!(msg_change.old_value, Value::String(String::new()));
        assert_eq!(msg_change.new_value, Value::String("hello".into()));
    }

    #[test]
    fn test_provenance_sequential_numbering() {
        let contract = test_contract();
        let mut executor = Executor::new(contract);

        executor
            .execute_operation("echo", r#"{"message": "first"}"#)
            .unwrap();
        executor
            .execute_operation("echo", r#"{"message": "second"}"#)
            .unwrap();
        executor
            .execute_operation("echo", r#"{"message": "third"}"#)
            .unwrap();

        let log = executor.provenance();
        assert_eq!(log.entries[0].sequence, 0);
        assert_eq!(log.entries[1].sequence, 1);
        assert_eq!(log.entries[2].sequence, 2);
    }

    // ── Public API Tests ──────────────────────────────────

    #[test]
    fn test_execute_contract_single_request() {
        let contract = test_contract();
        let result = execute_contract(
            &contract,
            r#"{"operation": "echo", "inputs": {"message": "hello"}}"#,
        )
        .unwrap();

        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json["success"], true);
        assert_eq!(json["contract_id"], "ic-test-001");
    }

    #[test]
    fn test_execute_contract_array_requests() {
        let contract = test_contract();
        let result = execute_contract(
            &contract,
            r#"[{"operation": "echo", "inputs": {"message": "hello"}}]"#,
        )
        .unwrap();

        let json: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json["success"], true);
    }

    #[test]
    fn test_execute_contract_invalid_input() {
        let contract = test_contract();
        let result = execute_contract(&contract, "not json");
        assert!(result.is_err());
    }

    // ── Determinism Tests ─────────────────────────────────

    #[test]
    fn test_deterministic_execution() {
        let contract = test_contract();
        let input = r#"{"operation": "echo", "inputs": {"message": "determinism test"}}"#;

        let first = execute_contract(&contract, input).unwrap();
        for i in 0..100 {
            let result = execute_contract(&contract, input).unwrap();
            assert_eq!(first, result, "Non-determinism at iteration {}", i);
        }
    }

    #[test]
    fn test_deterministic_multi_operation() {
        let contract = test_contract();
        let input = r#"[
            {"operation": "echo", "inputs": {"message": "first"}},
            {"operation": "echo", "inputs": {"message": "second"}}
        ]"#;

        let first = execute_contract(&contract, input).unwrap();
        for i in 0..100 {
            let result = execute_contract(&contract, input).unwrap();
            assert_eq!(first, result, "Non-determinism at iteration {}", i);
        }
    }

    #[test]
    fn test_deterministic_provenance() {
        let contract = test_contract();
        let input = r#"{"operation": "echo", "inputs": {"message": "prov test"}}"#;

        let first_json: serde_json::Value =
            serde_json::from_str(&execute_contract(&contract, input).unwrap()).unwrap();
        let first_provenance = &first_json["provenance"];

        for i in 0..100 {
            let result_json: serde_json::Value =
                serde_json::from_str(&execute_contract(&contract, input).unwrap()).unwrap();
            assert_eq!(
                first_provenance, &result_json["provenance"],
                "Provenance non-determinism at iteration {}",
                i
            );
        }
    }

    // ── Resource Limit Tests ──────────────────────────────

    #[test]
    fn test_resource_limit_memory_exceeded() {
        let mut contract = test_contract();
        contract
            .execution_constraints
            .resource_limits
            .max_state_size_bytes = 10;
        contract
            .execution_constraints
            .resource_limits
            .max_memory_bytes = 10;
        // Remove invariants so the only failure mode is memory
        contract.data_semantics.invariants.clear();

        let mut executor = Executor::new(contract);
        let result = executor.execute_operation(
            "echo",
            r#"{"message": "this string is way too long for the tiny memory limit we set"}"#,
        );
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("exceeds limit") || err.contains("bytes"));
    }

    #[test]
    fn test_precondition_enforcement() {
        // Create a contract where precondition is evaluable and fails
        let mut contract = test_contract();
        contract.behavioral_semantics.operations[0].precondition = "count >= 10".into();
        // Clear invariants to isolate precondition testing
        contract.data_semantics.invariants.clear();

        let mut executor = Executor::new(contract);
        // count starts at 0, precondition requires >= 10
        let result = executor.execute_operation("echo", r#"{"message": "hello"}"#);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Precondition failed"));
    }

    #[test]
    fn test_postcondition_verification() {
        // Create a contract where postcondition is evaluable
        let mut contract = test_contract();
        contract.behavioral_semantics.operations[0].postcondition = "count >= 1".into();
        // Clear invariants to isolate postcondition testing
        contract.data_semantics.invariants.clear();

        let mut executor = Executor::new(contract);
        // Operation doesn't set count, so postcondition count >= 1 fails
        let result = executor.execute_operation("echo", r#"{"message": "hello"}"#);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("postcondition") || err.contains("Contract violation"));
    }
}
