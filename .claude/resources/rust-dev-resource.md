# Rust Development Reference (Agent-Optimized)

## Metadata
- Sources: rust-lang.org/blog, github.com/rust-lang/rust/releases, lib.rs, crates.io
- Refresh: 2025-01-31
- Coverage: Jan 2025 baseline → Dec 2025 present
- Target: LLM agent consumption

## Language Evolution Post-Cutoff

### Edition 2024 (Stabilized Oct 2024, Rust 1.82+)
- `unsafe_op_in_unsafe_fn`: nested unsafe required
- `gen` blocks: inline generators `gen { yield expr }`
- RPIT lifetime capture: `impl Trait` captures all lifetimes
- `impl Trait` in bounds: `fn foo<T: impl Trait>()` → `fn foo(impl Trait)`
- `if let` chain guards: `match x { Some(y) if let Some(z) = other => }`
- Reserved syntax: `gen` keyword, `async gen` blocks

### Async/Await
- Async closures stable: `async |x| { ... }` (Rust 1.78+)
- Return-position impl Trait in traits (RPITIT): `async fn` in traits stable
- Async drop: unstable, `async_drop` feature gate
- Generic associated types (GATs): stable for async trait patterns

### Type System
- Type Alias Impl Trait (TAIT): `type Foo = impl Trait;` progressive stabilization
- Associated type bounds: `T: Iterator<Item: Display>` stable
- `dyn*`: experimental trait object without allocation
- Const generics: full feature parity approaching (complex expressions partial)
- Generic associated types (GATs): fully stable Rust 1.65+

### Pattern Matching
- Or-patterns in `let`: `let Ok(x) | Err(x) = result;`
- Subslice patterns: `[first, middle @ .., last]`
- Range patterns: `match x { 0..5 | 10..=15 => }`

### Error Handling
- Try blocks: `try { expr? }` unstable
- `Result::inspect` / `Option::inspect`: side-effect methods
- Error trait in core: `core::error::Error`

### Cargo & Tooling

#### Cargo Features (1.75-1.83)
- Package inheritance: workspace.package shared metadata
- Artifact dependencies: `artifact = "bin"` for build dependencies
- Weak dependency features: `foo?/feature` optional feature activation
- Cargo script: `cargo +nightly script.rs` single-file execution
- `--keep-going`: continue past first error
- Sparse registry default: faster index updates
- Lockfile v4: more stable merge resolution

#### Clippy/Rustfmt
- Cognitive complexity lints: nested logic detection
- `#![warn(clippy::pedantic)]`: stricter lint set
- Edition-specific linting: automatic migration assists
- Rustfmt: let-else, gen blocks formatting

#### Rust Analyzer
- Proc macro expansion server: faster IDE integration
- Incremental builds: workspace reuse improvements
- Native diagnostics: less reliance on `cargo check`

### Standard Library Additions

#### Collections
- `HashMap::extract_if`: conditional drain
- `BTreeMap::pop_first/pop_last`: ordered map ops
- `Vec::extract_if`: conditional element removal
- `slice::take_*`: split operations without panic

#### Concurrency
- `std::sync::OnceLock`: lazy static without macros
- Scoped threads: `std::thread::scope` stable
- `Mutex::blocking_lock_fair`: fairness guarantee

#### Iterators
- `Iterator::map_windows`: sliding window without collect
- `Iterator::intersperse`: element separation
- `array::from_fn`: generate arrays from closures

#### Primitives
- C-string literals: `c"string"` for FFI
- Offset-of macro: `core::mem::offset_of!(Type, field)`
- Pointer metadata APIs: wide pointer inspection
- `f16` / `f128`: half/quad precision (unstable)

#### IO/Filesystem
- `File::create_new`: atomic exclusive creation
- `IsTerminal` trait: TTY detection
- Owned fd types: `OwnedFd` on Unix

### Ecosystem Evolution

#### Major Crate Updates
- tokio 1.35+: cooperative task yielding improvements
- serde 1.0.200+: enum representation improvements
- clap 4.x: derive macro performance, better errors
- reqwest 0.12+: HTTP/3 support (experimental)
- diesel 2.2+: async support via `diesel-async`

#### New Prominent Crates
- axum: ergonomic web framework (tower-based)
- ratatui: TUI framework (fork of tui-rs)
- moka: high-performance caching
- rustls 0.23+: TLS 1.3, post-quantum readiness
- divan: benchmarking (competitor to criterion)

### Cross-Compilation & Targets
- wasm32-wasip2: WASI preview 2
- Apple silicon: tier 1 support stable
- ARM64 Windows: tier 2
- LoongArch64: tier 2/3 platforms

### Lints & Diagnostics
- `clippy::must_use_candidate`: suggest `#[must_use]`
- `clippy::uninlined_format_args`: format string captures
- `elided_lifetimes_in_paths`: edition migration warning
- Diagnostic namespaces: `#[diagnostic::on_unimplemented]`

## Common Patterns Post-Cutoff

### Async Trait Pattern
```rust
trait AsyncTrait {
    async fn method(&self) -> Result<T, E>;
}
// No longer requires async_trait macro in many cases
```

### Error Context Chaining
```rust
use anyhow::Context;
result.context("operation failed")?;
// Preferred over manual error wrapping
```

### Const Generic Arrays
```rust
fn process<const N: usize>(arr: [T; N]) -> [U; N]
// Array operations without Box/Vec overhead
```

### Match Ergonomics
```rust
match option_result {
    Some(Ok(value)) => process(value),
    Some(Err(e)) => handle(e),
    None => default(),
}
// Nested destructuring without ref/deref noise
```

## Build System Intelligence

### Feature Flag Patterns
- Default features: `default = ["std"]`
- Additive: `std = ["dep:std-crate"]`
- Exclusive: `#[cfg(not(all(feature = "a", feature = "b")))]`

### Workspace Optimization
```toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
# Inherit: serde = { workspace = true }
```

### Profile Tuning
```toml
[profile.release]
lto = "thin"
codegen-units = 1
strip = true
# Binary size vs compile time tradeoff
```

## Migration Gotchas

### Edition 2024 Breaking Changes
- `unsafe` nesting required: wrap unsafe ops in functions
- Lifetime capture: explicit `+ use<>` bounds needed for non-capture
- Keyword reservations: `gen`, `async gen` cannot be identifiers

### Dependency Hell Patterns
- Resolver v2: default in edition 2021+, check Cargo.toml
- Feature unification: shared deps unify features across workspace
- Version conflicts: `cargo tree -d` duplicates diagnostic

### FFI Evolution
- C-string: `c"literal"` vs `CString::new()` allocation
- Null pointers: `NonNull::new()` preferred over raw checks
- ABI stability: `#[repr(C)]` + careful layout for cross-lang

## Performance Primitives

### Lock-Free
- `std::sync::OnceLock`: init-once without mutex
- Atomic `fetch_update`: CAS loop abstraction
- `thread::scope`: structured concurrency zero-cost

### Allocation
- `Box::new_uninit()`: skip zero-init
- `Vec::with_capacity()`: pre-allocate
- `String::from_utf8_unchecked()`: skip validation when safe

### Compute
- SIMD: `std::simd` portable abstractions (unstable)
- Inline assembly: `asm!` macro stable
- Target features: `#[target_feature(enable = "avx2")]`

## Subagent Learning Repository

### Template Structure
```
[TIMESTAMP] [CONTEXT_HASH] [CATEGORY]
Discovery: <finding>
Pattern: <code or approach>
Constraint: <limitation or caveat>
Source: <verification method>
---
```

### Categories
- syntax: language feature usage
- stdlib: standard library API patterns
- ecosystem: crate integration techniques
- performance: optimization discoveries
- safety: unsafe code patterns
- ffi: foreign function interface
- async: concurrency patterns
- error: error handling evolution
- tooling: cargo/clippy/analyzer insights
- migration: edition/version upgrade paths

### Learning Entries
<!-- Subagent-populated section -->

---

## Quick Reference Syntax

### Lifetime Syntax
- `'a`: named lifetime
- `'static`: program duration
- `'_`: elided/inferred
- `+ 'a`: outlives bound
- `for<'a>`: HRTB higher-rank
- `+ use<'a>`: explicit capture (edition 2024)

### Macro Patterns
- `macro_rules!`: declarative
- `#[proc_macro]`: procedural function-like
- `#[proc_macro_derive]`: derive
- `#[proc_macro_attribute]`: attribute

### Trait Bounds
- `T: Trait`: single bound
- `T: Trait + Other`: multiple
- `where T: Trait`: clause
- `impl Trait`: RPIT/APIT
- `dyn Trait`: trait object
- `T::Assoc`: associated type projection

### Async Syntax
- `async fn`: async function
- `async {}`: async block
- `.await`: suspend point
- `async move {}`: capture ownership
- `async |x| {}`: async closure

---

## Verification Protocol

When uncertain about post-cutoff features:
1. Search rust-lang.org/blog for stabilization announcements
2. Check github.com/rust-lang/rust/blob/master/RELEASES.md
3. Query lib.rs or crates.io for ecosystem changes
4. Verify with `rustc --version` + feature gate testing

Update metadata.refresh when new information integrated.
