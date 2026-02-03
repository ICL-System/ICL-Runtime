# Intent Contract Language — Testing Guide

**Purpose:** Ensure ICL correctness through comprehensive testing at all levels.

---

## Test Pyramid

```
            / \
           /   \  System Tests
          /     \ (End-to-end workflows)
         /-------\
        /         \
       /   Integ   \  Integration Tests
      /   Tests     \ (Module interactions)
     /-----------\
    /             \
   /  Unit Tests   \  Unit Tests
  /________________\  (Individual functions)
```

---

## Level 1: Unit Tests

**Scope:** Individual functions, modules in isolation
**Coverage:** >90% of code
**Speed:** <100ms total

### Parser Unit Tests

```rust
#[cfg(test)]
mod parser_tests {
  use super::*;
  
  #[test]
  fn test_parse_empty_contract() {
    let text = "Contract {}";
    let result = parse_contract(text);
    assert!(result.is_ok());
  }
  
  #[test]
  fn test_parse_with_identity() {
    let text = r#"
      Contract {
        Identity {
          stable_id: "ic-001",
          version: 1,
          created_timestamp: 2026-02-01T10:00:00Z,
          owner: "test"
        }
      }
    "#;
    let contract = parse_contract(text).unwrap();
    assert_eq!(contract.identity.stable_id, "ic-001");
  }
  
  #[test]
  fn test_parse_invalid_syntax() {
    let text = "Contract {";  // Missing closing brace
    let result = parse_contract(text);
    assert!(result.is_err());
    if let Err(ParseError::Syntax { line, col, .. }) = result {
      assert!(line > 0);
      assert!(col >= 0);
    }
  }
  
  #[test]
  fn test_parse_unknown_field() {
    let text = r#"
      Contract {
        UnknownField {
          value: 123
        }
      }
    "#;
    let result = parse_contract(text);
    assert!(result.is_err());
  }
  
  #[test]
  fn test_parse_type_mismatch() {
    let text = r#"
      Contract {
        Identity {
          version: "not_an_integer"
        }
      }
    "#;
    let result = parse_contract(text);
    assert!(result.is_err());
  }
}
```

### Type Checker Unit Tests

```rust
#[test]
fn test_type_consistency() {
  let contract = Contract {
    data_semantics: DataSemantics {
      state: {
        count: Integer,
      },
      invariants: vec!["count >= 0".to_string()],
    },
  };
  
  let mut checker = TypeChecker::new();
  let result = checker.check(&contract);
  assert!(result.is_ok());
}

#[test]
fn test_type_mismatch_in_operation() {
  let contract = Contract {
    data_semantics: DataSemantics {
      state: { value: String },
    },
    behavioral_semantics: BehavioralSemantics {
      operations: vec![Operation {
        name: "increment".to_string(),
        computation: Some("value + 1".to_string()),  // String + 1 = type error
      }],
    },
  };
  
  let mut checker = TypeChecker::new();
  let result = checker.check(&contract);
  assert!(result.is_err());
}
```

### Normalizer Unit Tests

```rust
#[test]
fn test_normalization_is_idempotent() {
  let contract = parse_contract(TEST_CONTRACT).unwrap();
  
  let norm1 = normalize(&contract).unwrap();
  let norm2 = normalize(&norm1).unwrap();
  
  assert_eq!(norm1, norm2);
}

#[test]
fn test_normalization_removes_comments() {
  let contract_with_comments = r#"
    Contract {
      // This is a comment
      Identity {
        stable_id: "ic-001"  // Another comment
      }
    }
  "#;
  
  let contract = parse_contract(contract_with_comments).unwrap();
  let normalized = normalize(&contract).unwrap();
  
  // Serialize back and verify no comments
  let serialized = serde_json::to_string(&normalized).unwrap();
  assert!(!serialized.contains("//"));
}

#[test]
fn test_normalization_sorts_fields() {
  let contract1 = parse_contract(VARIANT_1).unwrap();
  let contract2 = parse_contract(VARIANT_2).unwrap();
  
  let norm1 = normalize(&contract1).unwrap();
  let norm2 = normalize(&contract2).unwrap();
  
  // Even though input differs, normalized forms should be identical
  assert_eq!(norm1, norm2);
}
```

---

## Level 2: Integration Tests

**Scope:** Multiple modules working together
**Coverage:** Happy paths + error paths
**Speed:** <1000ms total

### Parser → Verifier Integration

```rust
#[test]
fn test_parse_and_verify() {
  let contract = parse_contract(VALID_CONTRACT).unwrap();
  let result = verify(&contract);
  
  assert!(result.is_ok());
}

#[test]
fn test_parse_and_verify_invalid_contract() {
  let contract = parse_contract(INVALID_CONTRACT).unwrap();
  let errors = verify(&contract);
  
  assert!(!errors.is_empty());
  assert!(errors.iter().any(|e| e.kind == "NonDeterministic"));
}
```

### Normalizer → Executor Integration

```rust
#[test]
fn test_normalize_then_execute() {
  let contract = parse_contract(TEST_CONTRACT).unwrap();
  let canonical = normalize(&contract).unwrap();
  let inputs = create_test_inputs();
  
  let result = execute(&canonical, &inputs).unwrap();
  
  assert!(result.success);
}
```

### Full Pipeline Integration

```rust
#[test]
fn test_full_pipeline() {
  // 1. Parse
  let contract = parse_contract(FULL_TEST_CONTRACT).unwrap();
  
  // 2. Verify
  verify(&contract).unwrap();
  
  // 3. Normalize
  let canonical = normalize(&contract).unwrap();
  
  // 4. Execute
  let inputs = create_test_inputs();
  let result = execute(&canonical, &inputs).unwrap();
  
  // 5. Verify result
  assert!(result.success);
  assert!(result.contract_satisfied);
}
```

---

## Level 3: System Tests

**Scope:** End-to-end workflows
**Coverage:** Critical user paths
**Speed:** <5000ms per test

### Water Tracker Example

```python
import unittest
from icl import parse_contract, normalize, verify, execute

class WaterTrackerTest(unittest.TestCase):
  
  def setUp(self):
    self.contract_text = open('examples/water-tracker.icl').read()
    self.contract = parse_contract(self.contract_text)
  
  def test_create_log_entry(self):
    """Test: User logs water intake"""
    # Verify contract is valid
    verify(self.contract)
    
    # Normalize
    canonical = normalize(self.contract)
    
    # Execute: log 250ml intake
    result = execute(canonical, {
      'action': 'log_intake',
      'volume_ml': 250
    })
    
    # Assertions
    self.assertTrue(result.success)
    self.assertTrue(result.state_updated)
    self.assertEqual(result.output['total_ml'], 250)
  
  def test_daily_goal_tracking(self):
    """Test: System tracks progress toward 2L goal"""
    canonical = normalize(self.contract)
    
    # Log multiple times
    for _ in range(8):
      execute(canonical, {'action': 'log_intake', 'volume_ml': 250})
    
    # Query state
    final_result = execute(canonical, {'action': 'get_state'})
    self.assertEqual(final_result.output['total_ml'], 2000)
    self.assertTrue(final_result.output['goal_reached'])
  
  def test_immutability_of_past_entries(self):
    """Test: Past entries cannot be modified"""
    canonical = normalize(self.contract)
    
    # Log entry
    result1 = execute(canonical, {'action': 'log_intake', 'volume_ml': 250})
    entry_id = result1.output['entry_id']
    
    # Try to modify (should fail)
    result2 = execute(canonical, {
      'action': 'modify_entry',
      'entry_id': entry_id,
      'volume_ml': 500
    })
    
    self.assertFalse(result2.success)
    self.assertIn('immutable', result2.error)
```

---

## Level 4: Determinism Tests

**Guarantee:** Same input → identical output (100+ iterations)
**Criticality:** MANDATORY before deployment
**Speed:** <10000ms total (100 iterations)

### Determinism Test Template

```rust
#[test]
fn test_execution_determinism() {
  let contract = parse_contract(CONTRACT_TEXT).unwrap();
  let canonical = normalize(&contract).unwrap();
  
  let inputs = {
    "action": "log_intake",
    "volume_ml": 250
  };
  
  // Execute 100 times
  let mut results = Vec::new();
  for i in 0..100 {
    let result = execute(&canonical, &inputs)
      .expect(&format!("Execution failed at iteration {}", i));
    results.push(result);
  }
  
  // Verify all identical
  let first = &results[0];
  for (i, result) in results.iter().enumerate() {
    assert_eq!(
      first.output, result.output,
      "Output mismatch at iteration {}", i
    );
    assert_eq!(
      first.state_updated, result.state_updated,
      "State update mismatch at iteration {}", i
    );
  }
}
```

### Determinism Test Harness

```bash
#!/bin/bash
# Run determinism tests 100 times

echo "Testing determinism (100 iterations)..."

for i in {1..100}; do
  echo -n "."
  cargo test test_execution_determinism -- --nocapture > /tmp/det_$i.log
  if [ $? -ne 0 ]; then
    echo ""
    echo "FAILED at iteration $i"
    cat /tmp/det_$i.log
    exit 1
  fi
done

echo ""
echo "✓ All 100 iterations passed"
```

---

## Level 5: Property-Based Tests

**Approach:** Use fuzzing to find edge cases
**Tool:** Quickcheck or hypothesis

### Property: Normalization Idempotence

```rust
#[test]
fn prop_normalization_is_idempotent() {
  quickcheck::quickcheck! {
    fn check(contract_text: String) -> bool {
      // Skip invalid inputs
      let Ok(contract) = parse_contract(&contract_text) else {
        return true;
      };
      
      // Skip un-normalizable contracts
      let Ok(norm1) = normalize(&contract) else {
        return true;
      };
      
      // Check idempotence
      let Ok(norm2) = normalize(&norm1) else {
        return false;
      };
      
      norm1 == norm2
    }
  }
}
```

---

## Running Tests

### All tests

```bash
cargo test
npm test
```

### Specific test suite

```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# Determinism tests (100 iterations)
cargo test test_determinism

# With output
cargo test -- --nocapture
```

### With coverage

```bash
cargo tarpaulin --out Html
open tarpaulin-report.html
```

---

## Test Data

Store test contracts in `tests/fixtures/`:

```
tests/fixtures/
  valid/
    simple.icl
    water-tracker.icl
    reminder.icl
  invalid/
    syntax-error.icl
    type-error.icl
    unsatisfiable.icl
  performance/
    large-state.icl
    complex-operations.icl
```

---

## Failure Modes

**When a test fails, debug with:**

1. **Reproduce with minimal input**
   ```rust
   // Original failing test
   test_something()
   
   // Reduced to minimal case
   let minimal = TestCase {
     contract: CONTRACT_MINIMAL,
     inputs: INPUTS_MINIMAL,
   };
   ```

2. **Enable verbose output**
   ```bash
   cargo test test_name -- --nocapture
   ```

3. **Inspect intermediate states**
   ```rust
   println!("Before: {:?}", before);
   let result = operation();
   println!("After: {:?}", result);
   ```

4. **Determinism check**
   ```bash
   cargo test test_name --
 --test-threads=1
   ```

---

## Test Quality Checklist

Every test should:

- ✅ Test one thing
- ✅ Have descriptive name
- ✅ Include setup and teardown
- ✅ Verify both success and failure paths
- ✅ Use assertions with messages
- ✅ Be fast (<100ms for unit, <1s for integration)
- ✅ Be repeatable (no flakiness)
- ✅ Be isolated (no dependencies between tests)
- ✅ Have clear expected behavior documented

---

## Continuous Integration

**Tests run on every commit:**

```yaml
# .github/workflows/test.yml
name: Test
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all
      - name: Run determinism tests
        run: cargo test test_determinism
      - name: Upload coverage
        uses: codecov/codecov-action@v2
```

**No deployment without passing tests.**
