<div align="center">

# ⚡ Zet Lang

### The language that refuses to compile code it doesn't trust.

[![Version](https://img.shields.io/badge/v0.2.0-orange?style=flat-square&label=version)]()
[![License](https://img.shields.io/badge/CC_BY--NC--SA_4.0-red?style=flat-square&label=license)]()
[![Written In](https://img.shields.io/badge/Rust-black?style=flat-square&logo=rust)]()
[![Platform](https://img.shields.io/badge/Windows_x64-0078D6?style=flat-square&logo=windows)]()

**Compile-time taint analysis · Native speed · Structured concurrency · Zero runtime overhead**

[Quick Start](#-quick-start) · [Why Zet?](#-why-zet) · [Language Tour](#-language-tour) · [Benchmarks](#-benchmarks) · [Docs](DOCS.md)

</div>

---

## What is Zet?

**Zet** (Zero Trust) is a compiled programming language where **every piece of external data is untrusted by default** — and the compiler enforces it.

Network responses, user input, file reads — they're all born as `Untrusted` types. You literally cannot use them without passing through a `validate` block. Not at runtime. **At compile time.** Your binary never ships with an unvalidated input path.

Under the hood, Zet compiles to optimized native code via Rust — no VM, no garbage collector, no interpreter. On Fibonacci(40), it clocks in at **~240ms** — roughly 2× faster than Go, 50× faster than Python.

```
┌─────────────┐      ┌───────────────────┐      ┌──────────────┐      ┌──────────┐
│  .zt source  │ ──▶  │  Zet Compiler      │ ──▶  │  Rust codegen │ ──▶  │  Binary   │
│              │      │  taint + scope +   │      │  (optimized)  │      │  (native) │
│              │      │  determinism check │      │               │      │           │
└─────────────┘      └───────────────────┘      └──────────────┘      └──────────┘
```

---

## 🤔 Why Zet?

Most languages let you do this:

```python
# Python — runs fine, ships to production, gets hacked
user_input = input("Enter query: ")
db.execute(f"SELECT * FROM users WHERE name = '{user_input}'")  # 💀 SQL Injection
```

In Zet, **this doesn't compile:**

```zet
nondet fn main() -> Void {
    let query = call Console.read("Enter query: ")  // type: Untrusted
    println(query)  // ❌ COMPILE ERROR: tainted variable 'query' used without validation
}
```

You're forced to validate first:

```zet
nondet fn main() -> Void {
    let query = call Console.read("Enter query: ")

    validate query {
        success: {
            // 'query' is now a trusted String — safe to use
            println("User said: " + query)
        }
    }
}
```

This isn't a linter warning. It's not a "best practice." **The compiler won't produce a binary until you handle it.**

### The Four Pillars

| Pillar | What it means | Compile-time enforced? |
|--------|--------------|:---:|
| 🔒 **Zero Trust** | All external data is `Untrusted`. Must `validate` before use. | ✅ |
| ⚡ **Native Speed** | No VM, no GC. Compiles to optimized machine code via Rust. | — |
| 🧠 **Smart Engine** | `det` fns get pure codegen; `nondet` gets async. Mixing them is an error. | ✅ |
| 🧵 **Structured Concurrency** | `spawn` only works inside `scope` blocks. No zombie threads. Ever. | ✅ |

---

## 🚀 Quick Start

> **Requirements:** Windows x64, [Rust toolchain](https://rustup.rs/) installed.

### Option A — Download the installer
1. Grab the latest release from [Releases](https://github.com/cekYc/zet-lang-source/releases)
2. Right-click `kurulum.bat` → **Run as Administrator**
3. Open a new terminal and type `zet`

### Option B — Build from source
```bash
git clone https://github.com/cekYc/zet-lang-source.git
cd zet-lang-source
cargo build --release --bin zet-compiler
```

### Hello, Zet

Create `hello.zt`:
```zet
nondet fn main() -> Void {
    println("Hello from Zet!")
}
```

Run it:
```bash
zet hello.zt
```

Output:
```
[Zet Parser] 1 fonksiyon bulundu.
Hello from Zet!
```

---

## 📖 Language Tour

### Variables & Types

```zet
let name = "Zet"
let age = 25
let scores = [100, 95, 87]
let first = scores[0]
```

| Type | Description |
|------|------------|
| `i64` | 64-bit integer |
| `String` | UTF-8 text |
| `Array<T>` | Typed collection |
| `Untrusted` | Tainted external data — cannot be used without `validate` |
| `Void` | No return value |

### Functions: `det` vs `nondet`

Zet forces you to declare your function's purity. The compiler verifies it — and rejects violations:

```zet
// Pure function — CPU & memory only. Async I/O here = compile error.
det fn fibonacci(n: i64) -> i64 {
    if n <= 1 { return n }
    println("Computing...")  // print/println are allowed in det functions
    return fibonacci(n - 1) + fibonacci(n - 2)
}

// Impure function — networking, async I/O, side effects.
nondet fn fetch_data() -> Void {
    let response = call HTTP.get("https://api.example.com/data")
    validate response {
        success: {
            println("Got: " + response)
        }
    }
}
```

> You can also write `deterministic` / `nondeterministic` in full — both forms are accepted.

**Rejected at compile time:**
- Async I/O calls (`HTTP.get`, `Console.read`) inside a `det` function
- `call` keyword on a `det` function (pure functions don't need async)

### Taint Analysis (Zero Trust in Action)

Any data from `Console.read`, `HTTP.get`, or similar sources is `Untrusted`:

```zet
let input = call Console.read("Your name: ")  // type: Untrusted
let data = call HTTP.get("https://...")        // type: Untrusted

// Using 'input' or 'data' directly anywhere = COMPILE ERROR
// You MUST validate:

validate input {
    success: {
        // 'input' is now a clean String — taint removed
        println("Hello, " + input)
    }
}
```

Taint **propagates** — deriving a value from tainted data (JSON parsing, indexing, concatenation) produces another `Untrusted` value.

### Structured Concurrency

```zet
nondet fn main() -> Void {
    scope Network {
        spawn HTTP.get("https://api-1.com")
        spawn HTTP.get("https://api-2.com")
    }
    // Execution reaches here ONLY after ALL spawns in 'Network' have completed.
    // No dangling threads. No fire-and-forget. No zombies.

    println("All network calls done.")
}
```

`spawn` outside a `scope`? **Compile error.** A `scope` block collects every spawned task into a `JoinHandle` vec and awaits all of them before proceeding to the next line.

### The `call` Keyword

`call` awaits a nondeterministic operation inline:

```zet
let now = call Util.now()                // pauses this task, not the whole program
let page = call HTTP.get("https://...")  // async under the hood
let n = call Util.to_int("42")           // string → i64
```

Using `call` on a `det` function is a compile error — pure functions don't need async machinery.

---

## 📊 Benchmarks

**Fibonacci(40)** — naive recursive, no memoization:

| Language | Time | Relative |
|----------|------|----------|
| **Zet** | **~240ms** | **1.0×** |
| C (gcc -O2) | ~230ms | ~same |
| Rust | ~230ms | ~same |
| Go | ~480ms | 2.0× slower |
| Java | ~550ms | 2.3× slower |
| Node.js | ~1.2s | 5× slower |
| Python | ~12s | 50× slower |

> Compiled with `opt-level=3`, LTO, single codegen unit, `panic=abort`, symbol stripping.

---

## 🔧 Standard Library (v0.2)

| Module | Function | Returns | Description |
|--------|----------|---------|-------------|
| **Built-in** | `print(value)` | `Void` | Print to stdout (no newline) |
| **Built-in** | `println(value)` | `Void` | Print to stdout (with newline) |
| **Console** | `call Console.read(prompt)` | `Untrusted` | Read user input from terminal |
| **HTTP** | `call HTTP.get(url)` | `Untrusted` | Async HTTP GET request |
| **Util** | `call Util.now()` | `i64` | Current Unix timestamp in ms |
| **Util** | `call Util.to_int(s)` | `i64` | Parse string to integer |
| — | `json(data, key)` | `String` | Extract a field from JSON text |

---

## 🏗️ Compiler Architecture

```
src/
├── main.rs              # CLI entry & pipeline orchestrator
├── parser.rs            # Nom-based recursive descent parser
├── ast.rs               # AST node definitions
├── codegen.rs           # Rust code generation (preamble + per-function)
└── analysis/
    ├── taint.rs         # HashSet-based taint tracking & propagation
    ├── determinism.rs   # Purity enforcement with nondeterministic stdlib list
    └── scope.rs         # spawn-inside-scope validation
```

**Pipeline:** `.zt` → Parse → Taint Analysis → Determinism Check → Scope Validation → Rust Codegen → `cargo build` → Native Binary

---

## 🗺️ Roadmap

- [x] Compile-time taint analysis with propagation
- [x] Deterministic / Nondeterministic function enforcement
- [x] Structured concurrency (`scope` + `spawn` + `JoinHandle`)
- [x] HTTP client, JSON parsing, console I/O
- [x] Native performance (Fibonacci-40 in ~240ms)
- [ ] Pattern matching
- [ ] Custom struct types
- [ ] Module system & imports
- [ ] Linux / macOS support
- [ ] LSP for editor integration
- [ ] Package manager

---

## 📜 License

[CC BY-NC-SA 4.0](LICENSE) — Free for non-commercial use. Attribution required. Share-alike.

---

<div align="center">

**Zet doesn't trust your inputs. And neither should you.**

*Star the repo, clone it, and try writing something in `.zt` — you might be surprised how different it feels when the compiler actually has your back.*

</div>
