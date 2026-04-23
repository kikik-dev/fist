# Fist Framework

Fist is an ultra-high-performance, API-only backend framework written in Rust. Inspired by the "One Punch" philosophy, it aims to eliminate framework overhead, providing direct execution paths from route to service.

## 🚀 Performance
Built on `Axum`, `Tokio`, and `mimalloc`, Fist is designed for speed. 
- **Benchmark:** 37,000+ Requests Per Second (RPS) on local hardware.
- **Latency:** Sub-10ms average response time under standard load.

## 🥊 Features
- **Direct Routing:** Zero-overhead request execution.
- **Strict Payload Validation:** Type-safe DTOs with `#[derive(FistRequest)]`.
- **Developer Experience:** Integrated CLI for scaffolding and hot-reloading.
- **Observability:** Built-in `tracing` support.
- **Production Ready:** Centralized connection pooling and JWT middleware.

## 🏗 Project Structure
- `fist-core/`: The core engine and routing library.
- `fist-cli/`: CLI tool for scaffolding and development workflow.
- `fist-macros/`: Procedural macros for automated validation.
- `src/`: Your application-specific business logic.

## ⚡ Quick Start
1. **Initialize:** `fist new <project_name>`
2. **Develop:** `fist serve --watch`
3. **Build:** `fist build --release`

## License
MIT
