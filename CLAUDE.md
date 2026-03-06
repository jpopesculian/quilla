# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

Use `make` from the repo root — it delegates to all three crates:

```bash
make check   # clippy across all crates + feature combinations
make test    # tests across all crates + feature combinations
```

Per-crate shortcuts:

```bash
# crates/core (quilla)
make -C crates/core check   # clippy over all feature combinations
make -C crates/core test    # tests over all feature combinations

# crates/lang (quilla-lang)
make -C crates/lang check   # clippy over feature combinations (std, libm)
make -C crates/lang test    # tests over feature combinations

# crates/js (quilla-js)
make -C crates/js check     # cargo clippy
make -C crates/js test      # cargo test
make -C crates/js wasm      # build wasm32 release + wasm-bindgen + wasm-opt
```

Run a single test: `cargo test -p quilla test_name` (or `-p quilla-lang`, `-p quilla-js`).

## Architecture

Quilla is a quantum circuit simulator using state vector representation. Three crates:

- **`crates/core`** (`quilla`) — core Rust library, `no_std` compatible (feature-gated `std`)
- **`crates/lang`** (`quilla-lang`) — text-based circuit description language parser, `no_std` compatible
- **`crates/js`** (`quilla-js`) — wasm-bindgen bindings exposing core + lang to JavaScript, outputs to `pkg/`

### Core crate layers

1. **Gates** (`operations/`) — One file per gate, each implementing `StateVectorOperation<T>` for f32/f64. Simple gates convert to `UnitaryGate<T, N>` via `From` impls; some gates (CX, Measure, Swap) apply directly via bit operations.

2. **StateVector** (`state_vector.rs`) — Quantum amplitudes (`Vec<Complex<T>>`, size 2^qubits) + classical bits (`BitVec`). Operations applied via `apply()` / `apply_all()`.

3. **Circuit** (`circuit.rs`) — Generic `Circuit<O>` container. `DynCircuit` (= `Circuit<Box<dyn Operation>>`) provides a builder API: `.h(0).cx(0,1).meas(0,0)`. Sampling creates a fresh StateVector and applies all operations.

4. **Drawing** (`draw.rs`) — ASCII circuit diagram renderer. Gates implement `DrawOperation` trait to describe their visual representation.

### Lang crate pipeline

Parses text like `h 0\ncx 0 1\n` into `Instruction` values through a 4-stage pipeline:

1. **Lexer** (`lexer.rs`) — bytes → `Token` stream (idents, numbers, parens, newlines)
2. **ExprParser** (`expr/`) — tokens → `Expr` trees (handles parenthesized arithmetic with `+`, `-`, `*`, `/`)
3. **FuncParser** (`func/`) — expressions → `Func` (ident + evaluated `Num` args)
4. **Instruction** (`instruction.rs`) — `Func` → typed `Instruction` enum via `TryFrom`

Each stage is streaming: feed items in, pull results out, close when done. Errors are `Spanned<ParseError>` for source location tracking.

### Key traits

- `StateVectorOperation<T>` — core trait: `fn apply_to(&self, state: &mut StateVector<T>, rng: &mut DynRng)`
- `Operation` — compound trait (`StateVectorOperation<f64> + StateVectorOperation<f32> + DrawOperation + Debug`), auto-implemented via blanket impl
- `DrawOperation` — `fn draw_to(&self, d: &mut CircuitDrawing)`

### Adding a new gate

1. Create `crates/core/src/operations/{name}_gate.rs` with struct + `StateVectorOperation<T>` impl + `DrawOperation` impl
2. Export from `operations/mod.rs` and re-export from `lib.rs`
3. Add builder method on `DynCircuit` in `circuit.rs`
4. Add `Instruction` variant in `crates/lang/src/instruction.rs` and match arm in `TryFrom<Func>`
5. Add wasm-bindgen wrapper in `crates/js/src/` if needed
6. Add inline `#[cfg(test)]` tests in the gate file

## Conventions

- Rust 2024 edition; `no_std` compatible with feature-gated `std`
- Tests are inline `#[cfg(test)]` modules, not separate files
- Use `assert_complex_close()` from `num.rs` and local `basis_state()` helpers in gate tests
- Gate structs: `PascalCase` + `Gate` suffix; builder methods: short lowercase (`.h()`, `.cx()`)
- No `unsafe`, no macros — panics for invariant violations
