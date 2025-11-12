# Rust Module System & Visibility

**Context:** Learned while refactoring monolithic `lib.rs` into organized modules (Chapter 3.8)

## Core Principle: Explicit > Implicit

**Scala:** File structure = package structure (automatic)
**Rust:** Must explicitly declare every module with `mod` keyword

## Files ≠ Modules

```rust
// File exists: src/routes/health_check.rs
// ❌ NOT automatically a module!

// Must declare in parent (src/routes/mod.rs):
mod health_check;  // Now it's a module
```

**Scala equivalent:**
```scala
// File: routes/HealthCheck.scala
package routes  // Automatic from file location
```

## The `mod` Keyword - Two Jobs

### 1. Declare module exists
```rust
mod health_check;  // Rust looks for:
                   // - health_check.rs
                   // - health_check/mod.rs
```

### 2. Control visibility
```rust
mod health_check;      // PRIVATE (default)
pub mod health_check;  // PUBLIC
```

## Two-Layer Visibility

Both layers must be `pub` for external access:

```rust
// Layer 1: Module visibility (in routes/mod.rs)
pub mod subscriptions;  // ← Must be pub

// Layer 2: Item visibility (in routes/subscriptions.rs)
pub async fn subscribe() { }  // ← Must be pub
```

**Scala parallel:**
```scala
object subscriptions {        // Layer 1: object visibility
  def subscribe = ???          // Layer 2: member visibility
}
```

## `pub use` - Re-exports

Flatten module hierarchy for convenience:

```rust
// routes/mod.rs
pub mod health_check;
pub use health_check::*;  // Re-export everything
```

**Effect:**
- Without re-export: `use crate::routes::health_check::health_check`
- With re-export: `use crate::routes::health_check`

**Scala 3 equivalent:**
```scala
object routes {
  object health_check {
    def healthCheck = ???
  }
  export health_check._  // Re-export at package level
}
```

## Import Paths: `crate::` vs `cratename::`

### `crate::` - Relative (inside crate)

Used **within** the crate's own modules:

```rust
// In src/startup.rs (part of zero2prod crate)
use crate::routes::health_check;  // "From MY crate root"
```

### `cratename::` - Absolute (outside crate)

Used from **external** code importing the library:

```rust
// In src/main.rs (binary using library)
use zero2prod::routes::health_check;

// In tests/health_check.rs (test using library)
use zero2prod::startup::run;
```

**Mental model:**
- `crate::` = `this` (self-reference)
- `cratename::` = importing external dependency

## Special File: `mod.rs`

The "index" or "package object" of a module:

```
routes/
├── mod.rs           ← Module root (declares submodules)
├── health_check.rs
└── subscriptions.rs
```

**In `routes/mod.rs`:**
```rust
pub mod health_check;    // Declare submodules
pub mod subscriptions;

pub use health_check::*; // Re-export for convenience
pub use subscriptions::*;
```

## Standard Project Structure

```
src/
├── lib.rs              # Crate root
│   pub mod routes;     # Declares top-level modules
│   pub mod startup;
│   pub mod configuration;
│
├── main.rs             # Binary (uses library via cratename::)
│
├── startup.rs          # Module (uses crate:: for internal imports)
│
└── routes/
    ├── mod.rs          # Submodule root
    │   pub mod health_check;
    │   pub use health_check::*;
    │
    └── health_check.rs
        pub async fn health_check() { }
```

## Visibility Defaults

| Language | Default Visibility |
|----------|-------------------|
| Scala    | Public |
| Rust     | Private |

**Principle:** Rust is "secure by default" - must opt-in to public API.

## Key Differences: Scala vs Rust

| Aspect | Scala | Rust |
|--------|-------|------|
| **File = Module?** | Yes (automatic) | No (explicit `mod`) |
| **Default visibility** | Public | Private |
| **Package structure** | From directories | From `mod` declarations |
| **Re-exports** | `export` (Scala 3) | `pub use` |
| **Self-reference** | Package-relative imports | `crate::` |

## Common Gotchas

1. **Creating file doesn't create module** - Must declare with `mod`
2. **Making function `pub` isn't enough** - Module must also be `pub`
3. **Using `cratename::` inside crate** - Use `crate::` instead
4. **Forgetting `pub` on `mod`** - Module exists but inaccessible
