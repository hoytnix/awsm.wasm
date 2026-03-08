# RFC: AWSM v1.0 — Alchememe WebAssembly Mutation Language

- **Status**: Active Standard  
- **Version**: 1.0  
- **Authors**: Alchememe / Athanor team  
- **Target**: `wasm32-wasi` runtimes (KILN Sovereign Sanctuary)  
- **Date**: March 2026  

## 1. Abstract

AWSM v1.0 is a **total, sub-Turing, purely functional DSL** for deterministic AST mutations. It provides **1:1 semantic parity** with legacy TypeScript transformation tools while guaranteeing mathematical determinism via Stable Pointer Hashes (SPH) and the Patchiest Wire Schema (PWS) v1.1.

Every AWSM execution is a total function:  
$$ f(P, S) \rightarrow (P', T) $$  
where $$P$$ = Pristine State (AST topology), $$S$$ = AWSM bytecode, $$P'$$ = transformed state, $$T$$ = PWS transaction.

**Determinism Contract**: Identical Pristine State Hash + identical AWSM bytecode → identical PWS receipts across all conformant runtimes.

## 2. Stable Pointer Hash (SPH) Algorithm

### 2.1 Normalization Rules
AST nodes are normalized before hashing:

1. **Trivia Stripping**: Remove comments, whitespace-only tokens, non-semantic punctuation.  
2. **Node Signature** ($$S$$): $$S = \text{Kind} + \text{FieldName} + \text{NormalizedContent}$$

### 2.2 Hierarchical Hash Derivation
$$ H_{node} = \text{BLAKE3}(H_{parent} \parallel S) $$

- **Algorithm**: BLAKE3 (parallel, 256-bit collision resistance)  
- **Truncation**: `h12_hash` = first 12 hex chars of BLAKE3 output  
- **Canonical URN**: `urn:awsm:<file_path>:<anchor_id>/<h12_hash>`

**Example**:  
```
urn:awsm:src/main.rs:route_fn/a1b2c3d4e5f6
```

## 3. Patchiest Wire Schema (PWS) v1.1

### 3.1 Envelope Structure
```json
{
  "v": 1,
  "tx_id": "<sha256>",
  "actions": [ /* Action objects */ ],
  "purity_hash": "<blake3-final-state>"
}
```

### 3.2 Action Specifications

| Action                  | Required Fields                          | Constraints & Semantics |
|-------------------------|------------------------------------------|-------------------------|
| **`MUTATE_CALL`**      | `target_urn`, `payload`                 | `payload` requires **at least one** of: `rename` (String), `inject_args` (Map), `target_arg_index` (uint). |
| **`MANAGE_IMPORT`**    | Variable (`ensure` OR `replace_with`)   | Either `ensure {module}` or `replace_with {target_import, module}`. |
| **`TRANSLATE_DIALECT`** | `target_urn`, `enforce_explicit_type`   | Type safety enforcement. Optional: `generate_interface` (Array), `target_param_index` (uint). |
| **`RESTRUCTURE_TOPOLOGY`** | `target_urn`, `hardcoded_dependency`, `extract_to_parameter` | Converts `new X()` → injected DI parameter. |

#### 3.2.1 `MUTATE_CALL` Example
```json
{
  "type": "MUTATE_CALL",
  "target_urn": "urn:awsm:src/api.rs:route_fn/a1b2c3d4e5f6",
  "payload": {
    "rename": "handle_request_safely",
    "inject_args": { "timeout": "30" },
    "target_arg_index": 0
  }
}
```

#### 3.2.2 `MANAGE_IMPORT` Example
```json
{
  "type": "MANAGE_IMPORT",
  "ensure": "core.exceptions",
  "module": "with_standard_error_handling"
}
```

#### 3.2.3 `TRANSLATE_DIALECT` Example
```json
{
  "type": "TRANSLATE_DIALECT",
  "target_urn": "urn:awsm:src/types.ts:UserInterface/123def456abc",
  "enforce_explicit_type": true,
  "generate_interface": ["id: string", "name: string", "email: string"],
  "target_param_index": 1
}
```

#### 3.2.4 `RESTRUCTURE_TOPOLOGY` Example
```json
{
  "type": "RESTRUCTURE_TOPOLOGY",
  "target_urn": "urn:awsm:src/service.ts:UserService/789ghi012jkl",
  "hardcoded_dependency": "new DatabaseClient()",
  "extract_to_parameter": "dbClient"
}
```

**Versioning**: All payloads use `"v": 1`. Backward-compatible extensions maintain v1 parity.

## 4. AWSM-WASI ABI Contract

### 4.1 Host Functions (`extern "C"`)
```c
int32_t awsm_read_topology(uint8_t* ptr, int32_t len);
int32_t awsm_validate_ast(const uint8_t* ptr, int32_t len);  
void    awsm_log_event(const uint8_t* ptr, int32_t len);
```

#### `awsm_read_topology`
- **Input**: Pre-allocated buffer (`ptr`, `len`)  
- **Output**: Bytes written to topology map. Returns `> len` → retry with larger buffer.

#### `awsm_validate_ast`  
- **Returns**: `0` = Pure, `1` = Syntax Error, `2` = Scope Violation

#### `awsm_log_event`
**Envelope**:
```json
{ "level": "INFO|WARN|ERROR", "module": "awsm-vm", "data": {} }
```

### 4.2 Exit Codes
| Code  | Name              | Semantics |
|-------|-------------------|-----------|
| `0x00`| `SUCCESS`         | PWS emitted |
| `0x10`| `ERR_READ_DENIED` | Outside grants |
| `0x20`| `ERR_PURITY_FAIL` | Syntax/policy failure |
| `0x30`| `ERR_GAS_EXHAUSTED` | Depth/gas limit |

## 5. Execution Model

### 5.1 Code Tape Primitives (8 Instructions)
```
> selector     # Seek to AST node(s)
<             # Root pointer
+ symbol      # Mark anchor  
- symbol      # Unmark anchor
[ expr ]      # Sub-transaction scope (max depth: 32)
.             # Extract node signature  
, value       # Ingest metadata  
]             # Quench (merge to parent)
```

### 5.2 Transactional Semantics
```
Global Scope [0]
  ↓ [ push new scope ]
Nested Scope [1]     ← Local actions
  ↓ ] quench/merge
Global Scope [0]     ← Merged actions
```

**Quench Contract**: Single global scope → ordered `actions[]` → PWS JSON.

## 6. Total Determinism Requirements

**Given**:
- Identical **Pristine State Hash** (AST topology BLAKE3)
- Identical **AWSM Bytecode**  
- Identical **Recipe Metadata**

**All conformant VMs MUST emit**:
- Bit-for-bit identical **PWS JSON**  
- Identical **exit code**

**Prohibited**:
- Time, randomness, network IO  
- Uncontrolled filesystem access  
- Non-deterministic host calls

## 7. Conformance Criteria

An implementation is **AWSM v1.0 conformant** if it:

1. ✅ Implements SPH algorithm + URN format  
2. ✅ Emits PWS v1.1 with action constraints  
3. ✅ Respects AWSM-WASI ABI + exit codes  
4. ✅ Passes shared conformance suite  
5. ✅ Guarantees determinism contract

## 8. Backward Compatibility Matrix

| PWS Version | AWSM VM | Rust `patchiest` | TypeScript Tools |
| ----------- | ------- | ---------------- | ---------------- |
| v1.0        | ✅       | ✅                | ✅ (semantic)     |
| v1.1        | ✅       | ✅                | ✅ (semantic)     |

**AWSM v1.0 severs dependency on stochastic LLM text generation while maintaining 100% semantic equivalence with existing transformation pipelines.**