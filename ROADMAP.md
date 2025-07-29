# Phazor Roadmap

This roadmap outlines the major milestones and architectural goals for the Phazor project — a Rust + WASM single-page application framework designed for future Pythonic integration.

## ✅ MVP Milestones

| Phase | Goal | Status |
|-------|------|--------|
| 1 | SPA framework with `.phz` view files and routing via Yew | ✅ Complete |
| 2 | Tree-sitter grammar and parser for `.phz` files | ✅ Complete |
| 3 | Code generation for components and router module | ✅ Complete |

## 🚧 In Progress

| Phase | Goal | Status |
|-------|------|--------|
| 4 | Add `Props` support and basic reactive state via Yew hooks | 🚧 In Progress |
| 5 | Implement message outbox queue for local-only operation | 🔜 Next |
| 6 | Detect online/offline status and trigger sync attempts | 🔜 Next |

## 🔜 Planned Enhancements

| Phase | Goal |
|-------|------|
| 7 | Trait-based `DataSink` abstraction for pluggable backends (e.g. SQLite, REST, PostgreSQL) |
| 8 | Add support for inputs, forms, and two-way data binding |
| 9 | Build reusable ViewModel layer for separating logic from views |
| 10 | Add backend sync server with API endpoints (e.g. Axum or Actix) |
| 11 | Integrate optional authentication and per-user state |
| 12 | Optimize for offline-first PWA deployment |
| 13 | Provide dev tooling or simple CLI for scaffolding |

---

## 🧠 Vision

Phazor is being designed with the following principles:
- 🧩 Modular and extensible
- 🚀 Fast, native-feeling SPA
- 🔌 Backend-agnostic
- 🌐 WASM-first, Python-compatible in the future
- 🧠 MVVM-aligned architecture

This document evolves as the project grows. PRs and ideas welcome!

