# ⚡ isobrowsecli (`iso`)

> **The headless, terminal-native execution core of the [IsoBrowse](https://github.com/igtumt/isobrowse) ecosystem.**

`isobrowsecli` (`iso`) is a lightweight, zero-dependency WASI pipeline runner built with Rust and Wasmtime. It strips away all UI overhead to deliver a fast, sandboxed WebAssembly execution engine designed for command-line workflows, Unix pipelines, pre-commit hooks, and CI/CD automation.

---

## 🎯 Purpose & Ecosystem

While **[IsoBrowse](https://github.com/igtumt/isobrowse)** provides a local-first desktop interface for visual pipeline orchestration, **`isobrowsecli`** serves as its lightweight CLI counterpart. It brings the same isolated WASM execution model directly to your terminal and headless environments.

---

## ✨ Core Capabilities

- **📦 Zero External Dependencies:** Single compiled binary—no Node.js, Python, or Docker runtime required.
- **🔒 Sandboxed WASI Execution:** Executes untrusted `.wasm` binaries in isolated environments with strict boundaries.
- **🌊 Native Unix Piping:** Integrates smoothly with standard input (`stdin`) and standard output (`stdout`) streams.
- **⚡ Local Module Caching (`.isocache`):** Fetches remote WASM modules over HTTPS once and caches them locally for millisecond execution times.
- **🚥 Exit Code Propagation:** Intercepts WASI `exit(code)` calls gracefully and forwards status codes to the host shell—perfect for Git hooks and CI/CD gates.


---

## 🚀 Quick Start

### 1. Direct Execution
Run local or remote `.wasm` modules with arguments:
```bash
iso run path/to/worker.wasm "input_data"
```

### 2. Remote WASM Execution (Auto-cached)
Execute WASM modules directly from HTTPS endpoints:
```bash
iso run https://raw.githubusercontent.com/user/repo/main/analyzer.wasm "data"
```

### 3. Unix Pipeline (Stdin / Stdout)
Stream data through WASM workers using standard Unix pipes:
```bash
cat application.log | iso run https://.../log-cleaner.wasm > output.json
```

---

## 📥 Installation

Install the pre-compiled binary for macOS or Linux with a single command:

```bash
curl -fsSL [https://raw.githubusercontent.com/igtumt/isobrowsecli/main/install.sh](https://raw.githubusercontent.com/igtumt/isobrowsecli/main/install.sh) | sh
```

---

## 🛠️ Building from Source

Ensure you have the Rust toolchain installed, then clone and build:

```bash
git clone https://github.com/igtumt/isobrowsecli.git
cd isobrowsecli
cargo build --release
```

The compiled executable will be available at `./target/release/iso`.

---

## 🔗 Related Projects

- **[isobrowse](https://github.com/igtumt/isobrowse):** Local-first desktop application for visual WASM pipeline orchestrations.

