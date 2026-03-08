# Technical Context

## Technologies Used
- **WebAssembly (WASM)**: Specifically targeting `wasm32-wasi` to run in restricted sandbox environments (KILN Sovereign Sanctuary).
- **Rust**: For tooling and targeting the host VM environment, utilizing cargo standard structures and unit test harnesses.
- **BLAKE3**: Used for parallel, high-speed cryptographic hashing for the Stable Pointer Hash (SPH) algorithm.
- **JSON & YAML**: JSON used as the serialization format for Patchiest Wire Schema (PWS) envelopes. YAML (`serde_yaml`) used natively to scaffold Barren-Host Conformance Test Suite (BH-CTS) assertions.

## Development Setup
- Target environments must support the strictly defined AWSM-WASI ABI which includes `extern "C"` functions for reading topology, validating ASTs, and logging.

## Technical Constraints
- **Strict Determinism**: Zero access to system time, random number generators, or uncontrolled network/file IO. All computations must be 100% deterministic based solely on inputs.
- **Gas/Depth Limits**: The execution model enforces limits on depth (max depth: 32 for scopes) and gas to prevent unbounded execution (yielding `ERR_GAS_EXHAUSTED`).
- **Memory Management**: The ABI uses specific buffer writing mechanics (e.g., `awsm_read_topology` requiring callers to pre-allocate buffers and handle resizing if the topology size exceeds the initial buffer length).
