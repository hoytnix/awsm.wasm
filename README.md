<div align="center">
  <img src="https://awsm-wasm.netlify.app/awsm.svg" alt="AWSM Logo" width="200"/>
</div>

# ⚡️ AWSM (Alchememe WebAssembly Mutation Language) v1.0

![Coverage](https://img.shields.io/badge/Coverage-86.10%25-brightgreen.svg)
![Target](https://img.shields.io/badge/Target-wasm32--wasi-purple.svg)
![Status](https://img.shields.io/badge/Status-Active_Standard-blue.svg)

> **Total, sub-Turing, purely functional DSL for deterministic AST mutations.** 
> Replacing stochastic LLM hallucinations with mathematically verifiable precision.

---

## 🚀 The Mission

**AWSM** exists to provide a mathematically deterministic way to perform Abstract Syntax Tree (AST) mutations. We are severing the dependency on unpredictable, non-deterministic LLM text generation and upgrading to a purely functional execution model inside highly secure, parallelizable WebAssembly (`wasm32-wasi`) runtimes.

If you want code generation that hallucinates, go back to ChatGPT. If you want **1:1 semantic parity** with strict **cryptographic topological lineage**, you are in the right place.

## 🧠 Core Architecture

### 1. The Determinism Contract 🤝
Every execution of AWSM is a total function:
`f(P, S) -> (P', T)`  
(*P* = Pristine State, *S* = AWSM Bytecode, *P'* = Transformed State, *T* = PWS Transaction)

Given the exact same Pristine State Hash and AWSM Bytecode, **ALL conformant VMs MUST emit bit-for-bit identical JSON and exit codes.**

### 2. Stable Pointer Hash (SPH) 🔗
BLAKE3-powered parallel cryptographic hashing for AST nodes.
- Strips all non-semantic trivia (whitespace, comments).
- Derives hierarchical lineage (`H_node = BLAKE3(H_parent || S)`).
- Canonical URN generation: `urn:awsm:src/main.rs:route_fn/a1b2c3d4e5f6`

### 3. Patchiest Wire Schema (PWS) v1.1 📦
Strict JSON enveloping for transaction receipts. Includes rigorous type validations for payload executions like:
- `MUTATE_CALL`
- `MANAGE_IMPORT`
- `TRANSLATE_DIALECT`
- `RESTRUCTURE_TOPOLOGY`

### 4. Zero-Trust Execution 🚫
Built exclusively for `wasm32-wasi`.
- **NO** Time access.
- **NO** Random Numbers.
- **NO** Network IO.
- **NO** Infinite Loops (Depth limited to 32 transaction scopes).

## 🛠 Usage

AWSM operates primarily as an AOT (Ahead-of-Time) transformation compiler. Provide your mutation recipe and watch it quench perfectly into PWS JSON.

```lisp
;; RECIPE: standardize-api-errors
(awsm:pipe (<)
    (awsm:manage-import :ensure "core.exceptions" :module "with_standard_error_handling" :is-type false)
    (awsm:mutate-call :target @route_fn :inject {:decorators ["with_standard_error_handling"]})
)
```

Pipe it into the runner:
```bash
cat recipe.awsm | awsm
```

Output:
```json
{
  "v": 1,
  "tx_id": "pending_tx",
  "actions": [
    {
      "type": "MANAGE_IMPORT",
      "ensure": "core.exceptions",
      "module": "with_standard_error_handling",
      "is_type_only": false
    },
    {
      "type": "MUTATE_CALL",
      "target_urn": "urn:awsm:ROOT",
      "payload": {
        "inject_args": {
          "decorators": "with_standard_error_handling"
        }
      }
    }
  ],
  "purity_hash": "verified-awsm-state"
}
```

## 🧪 Barren-Host Conformance Test Suite (BH-CTS)

The project includes an embedded TDD harness validating the pure mathematical operations of the Lexer, Parser, and VM against rigorous YAML layout fixtures.

**Current Test Coverage: 86.10%** across all vital execution and parsing modules (`lexer.rs`, `parser.rs`, `vm.rs`, `abi.rs`, `sph.rs`, `pws.rs`).

## ⚙️ Development

```bash
# Run the test suite natively
cargo test

# Generate coverage configuration
cargo tarpaulin --ignore-tests
```

---
*Built by the Alchememe Team. 2026.*
