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
|   | → [Support route parameters like `/user/:id`](#route-parameter-support) | ✅ Starting |
|   | → Auto-generate prop structs for views | ✅ Done |
|   | → Parse and inject dynamic HTML with `props.name`, etc. | ✅ Done |
|   | → Create test module for Tree-sitter debug output | ✅ Done |
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

---

## 🔍 Route Parameter Support

### Goal
Allow `.phz` files to define routes with dynamic segments like `@route "/user/:id"`, and auto-generate the correct `#[at("/user/:id")]` Yew router entry and component prop.

### Steps
1. ✅ Update Tree-sitter grammar to allow parsing of `@route "/:param"` format.
2. ✅ Enhance parser to extract route parameters (e.g., `id`) and inject them into the `View` model.
3. ✅ Ensure props from the route are included in the Yew component.
4. ✅ Add logic to `write_component()` to declare props accordingly.
5. ✅ Test with `.phz` view:
   ```phz
   @route "/user/:id"
   @props id
   def user():
       <h1>User ID: {id}</h1>
