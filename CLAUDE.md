# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                          # Debug build
cargo test --lib                     # Run all tests (40 tests)
cargo test --lib -- <filter>         # Run tests matching filter (e.g. "hadamard")
cargo clippy                         # Lint (draw.rs has expected warnings - WIP module)
cargo fmt --check                    # Check formatting
```

Feature compatibility: `cargo hack check --feature-powerset` (tests all combinations including no_std and std)

## Architecture

Quilla is a quantum circuit simulator using state vector representation. Single crate workspace: `crates/core` (`quilla-core`).

### Three layers

1. **Gates** (`operations/`) - One file per gate. Each gate implements `StateVectorOperation<T>` for both f32 and f64. Simple gates manipulate state directly via bit operations; parametric gates (RX, RY, RZ) convert to `UnitaryGate<T, N>` for matrix application.

2. **StateVector** (`state_vector.rs`) - Holds quantum amplitudes (`Array1<Complex<T>>`, size 2^qubits) and classical bits (`BitVec`). Operations are applied via `apply()` / `apply_all()`.

3. **Circuit** (`circuit.rs`) - Generic `Circuit<O>` container. `DynCircuit` (= `Circuit<Box<dyn Operation>>`) provides a builder API: `.h(0).cx(0,1).meas(0,0)`. Sampling methods (`sample`, `sample_once`) create a fresh StateVector and apply all operations.

### Key traits

- `StateVectorOperation<T>` - core trait: `fn apply_to(&self, state: &mut StateVector<T>, rng: &mut DynRng)`
- `Operation` - compound trait requiring `StateVectorOperation<f64> + StateVectorOperation<f32>`, auto-implemented via blanket impl

### Adding a new gate

1. Create `crates/core/src/operations/{name}_gate.rs` with struct + `StateVectorOperation<T>` impl
2. Export from `operations/mod.rs`
3. Re-export from `lib.rs`
4. Add builder method on `DynCircuit` in `circuit.rs`
5. Add tests in `#[cfg(test)]` module within the gate file

## Conventions

- Rust 2024 edition, `no_std` compatible (feature-gated `std`)
- Tests are inline `#[cfg(test)]` modules, not separate test files
- Use `basis_state()` and `assert_complex_close()` helpers from `complex.rs` in tests
- Gate structs use `PascalCase` + `Gate` suffix; builder methods are short lowercase (`.h()`, `.cx()`, `.meas()`)
- Parametric gates store angles as generic `T`, with `From` impls for f32/f64 conversions to `UnitaryGate`
- No `unsafe`, no macros, no custom error types - panics for invariant violations
