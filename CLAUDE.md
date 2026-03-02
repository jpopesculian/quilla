# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

Use `make` from the repo root — it delegates to both crates:

```bash
make check   # clippy across all crates + feature combinations
make test    # tests across all crates + feature combinations
```

Per-crate shortcuts:

```bash
# crates/core (quilla-core)
make -C crates/core check   # clippy over all feature combinations
make -C crates/core test    # tests over all feature combinations

# crates/js (quilla-js)
make -C crates/js check     # cargo clippy
make -C crates/js test      # cargo test
make -C crates/js wasm      # build wasm32 release + wasm-bindgen + wasm-opt
```

## Architecture

Quilla is a quantum circuit simulator using state vector representation. Two crates:

- **`crates/core`** (`quilla-core`) — core Rust library, `no_std` compatible (feature-gated `std`)
- **`crates/js`** (`quilla-js`) — wasm-bindgen bindings exposing the core to JavaScript, outputs to `pkg/`

### Core layers

1. **Gates** (`operations/`) - One file per gate, each implementing `StateVectorOperation<T>` for f32/f64. Simple gates use bit operations; parametric gates (RX, RY, RZ) convert to `UnitaryGate<T, N>`.

2. **StateVector** (`state_vector.rs`) - Quantum amplitudes (`Array1<Complex<T>>`, size 2^qubits) + classical bits (`BitVec`). Operations applied via `apply()` / `apply_all()`.

3. **Circuit** (`circuit.rs`) - Generic `Circuit<O>` container. `DynCircuit` (= `Circuit<Box<dyn Operation>>`) provides a builder API: `.h(0).cx(0,1).meas(0,0)`. Sampling creates a fresh StateVector and applies all operations.

### Key traits

- `StateVectorOperation<T>` — core trait: `fn apply_to(&self, state: &mut StateVector<T>, rng: &mut DynRng)`
- `Operation` — compound trait (`StateVectorOperation<f64> + StateVectorOperation<f32>`), auto-implemented via blanket impl

### Adding a new gate

1. Create `crates/core/src/operations/{name}_gate.rs` with struct + `StateVectorOperation<T>` impl
2. Export from `operations/mod.rs` and re-export from `lib.rs`
3. Add builder method on `DynCircuit` in `circuit.rs`
4. Add wasm-bindgen wrapper in `crates/js/src/` if needed
5. Add inline `#[cfg(test)]` tests in the gate file

## Conventions

- Rust 2024 edition; `no_std` compatible with feature-gated `std`
- Tests are inline `#[cfg(test)]` modules, not separate files
- Use `basis_state()` and `assert_complex_close()` helpers from `complex.rs` in tests
- Gate structs: `PascalCase` + `Gate` suffix; builder methods: short lowercase (`.h()`, `.cx()`)
- No `unsafe`, no macros, no custom error types — panics for invariant violations
