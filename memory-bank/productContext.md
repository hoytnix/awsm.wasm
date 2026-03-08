# Product Context

## Why this project exists
AWSM (Alchememe WebAssembly Mutation Language) v1.0 exists to provide a mathematically deterministic way to perform Abstract Syntax Tree (AST) mutations. It replaces stochastic, non-deterministic LLM text generation with a purely functional, sub-Turing Domain Specific Language (DSL).

## Problems it solves
- **Non-determinism in code generation/mutation**: LLMs and other heuristics can produce unpredictable results when mutating codebases. AWSM guarantees that the same pristine state and bytecode will always produce the exact same mutation transactions.
- **Semantic Parity**: It ensures 1:1 semantic parity with legacy TypeScript transformation tools while upgrading the execution to a highly secure, parallelizable WebAssembly runtime.

## How it should work
- **Pure Function execution**: Every execution takes a Pristine State (AST topology) and AWSM bytecode, outputting a transformed state and a Patchiest Wire Schema (PWS) transaction.
- **Standardized Communication**: It uses PWS v1.1 for standardizing mutation actions like `MUTATE_CALL`, `MANAGE_IMPORT`, `TRANSLATE_DIALECT`, and `RESTRUCTURE_TOPOLOGY`.

## User experience goals
- Complete predictability and reliability for developers utilizing automated code mutations.
- Fast, secure execution via WASM standard (`wasm32-wasi`) isolated runtimes.
- Clear traceability of AST changes through canonical URNs and BLAKE3 hashing.
