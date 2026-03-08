# System Patterns

## System Architecture
AWSM v1.0 operates on a strict functional transformation pipeline:
1. **Input**: Pristine AST Topology + AWSM Bytecode + Recipe Metadata.
2. **Execution**: WASM Virtual Machine conforming to AWSM-WASI ABI.
3. **Output**: Patchiest Wire Schema (PWS) JSON and strict exit codes.

## Key Technical Decisions
- **Total, sub-Turing DSL**: The DSL is intentionally restricted to ensure code operates as a total function without infinite loops.
- **Stable Pointer Hash (SPH)**: Uses BLAKE3 parallel hashing for 256-bit collision resistance. AST nodes are stripped of trivia (comments, whitespace) and normalized before hashing.
- **PWS v1.1 Envelope Structure**: Standardized JSON format for transaction receipts ensuring backward compatibility with TypeScript tools and Rust `patchiest`.
- **AWSM-WASI ABI**: A tightly defined set of host functions (`awsm_read_topology`, `awsm_validate_ast`, `awsm_log_event`) restricting VM capabilities. Prohibited operations include time, randomness, and uncontrolled IO.

## Design Patterns in Use
- **Action-based Mutations**: Operations are categorized into discrete action payloads (e.g., `MUTATE_CALL`, `RESTRUCTURE_TOPOLOGY`).
- **Hierarchical Hash Derivation**: Child nodes derive their hashes partly from their parent's hash, ensuring strong cryptographic lineage of AST structures.
- **Transactional Semantics**: Scope-based execution model where local actions quench/merge into a global global ordered action array.
