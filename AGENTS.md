# AGENTS.md

Cross-tool agent instructions for any AI coding assistant working on this repository. Read this entirely before writing any code.

## 1. Idiomatic Immediate-Mode State

As a UI component library for `egui`, we do not have a global application state.

- **Stateless by Default:** Components should be as pure and stateless as possible. Take state via mutable references (e.g., `&mut bool` for toggles, `&mut String` for text inputs).
- **Minimal Retained State:** If a component *must* retain state across frames (like an animation progress or internal scroll state), use `ui.memory_mut()` keyed by `egui::Id`. Do not introduce global static state.

## 2. Explicit Decisions & Zero Ambiguity

Think deeply before you write code. All architectural, visual, and layout decisions must be explicitly reasoned out. If a requirement is ambiguous, state your assumption clearly or ask for clarification before proceeding. Do not guess.

## 3. Every Line of Code is a Liability

Code requires understanding, testing, maintaining, and debugging.

- **No Speculative Engineering:** Do not build abstractions, features, or configuration keys "just in case." Solve only the immediate problem using the simplest, most direct approach. Keep it simple.
- **Delete Whenever Possible:** Strive for minimalism. Dead, unused, or replaced code must be completely removed from the codebase, never bypassed or commented out.

## 4. Strict Architectural Boundaries

Respect the boundaries defined by the library's traits and feature flags.

- **The `Elegant` Trait:** This is the core marker and sizing hint trait. All components must implement it.
- **Leaf Components vs Containers:** Leaf components (buttons, badges) should implement `egui::Widget`. Container components (cards, accordions) should provide a `.show(&mut Ui, closure)` pattern.
- **Feature Flags:** Code relying on `egui_flex` must be strictly gated behind the `flex` feature. Ensure the library compiles cleanly with all feature permutations.

## 5. Builder Patterns and Type Safety

Design the codebase to be highly predictable and ergonomic for the consumer.

- **Builder Pattern:** Use builder patterns for component initialization (e.g., `ElegantButton::new("Text").primary().disabled(true)`).
- **Type Safety:** Use Rust's type system to enforce valid configurations. Don't use strings when enums will do.

## 6. Zero-Panic Policy (Crash Resilience)

As a library, we must **never** crash the host application.

- **No Unhandled Exceptions:** Never use `unwrap()` or `expect()` in rendering code.
- **Graceful Fallbacks:** If invalid input is provided, render a visual fallback, return an `egui::Response` with an error state, or log a warning, but do not panic.

## 7. Zero-Defect Safety

- **No Undefined Behavior or Data Races:** Rely on Rust's borrow checker and type system. Do not use `unsafe` blocks under any circumstances.
- **Clippy:** Treat `cargo clippy` warnings as errors. Ensure `#![warn(clippy::all)]` is respected.

## 8. Visual State Completeness

While the library doesn't execute asynchronous operations, it must provide the visual vocabulary for them.

- **Comprehensive States:** Components must support states like `disabled`, `loading`, `success`, and `error`.
- **Smooth Transitions:** Ensure smooth micro-animations when transitioning between these states (using `ui.ctx().animate_bool()` or similar helpers).

## 9. Git Workflow & Holistic Atomic Commits

- **Atomic Commits:** Every commit must be a single, self-contained logical unit (one feature, one bug fix, or one refactor).
- **Independently Revertible:** Every single commit in the history must be independently revertible without breaking the library.
- **Complete Context:** A code change commit must simultaneously include all relevant **example updates** and **documentation updates**. Do not defer them.

## 10. Tooling and Environment Baseline

- **Standard Cargo:** Use standard Cargo tooling.
- **Validation:** Before finishing a task, ensure the crate compiles and validates successfully via `cargo check --all-features` and `cargo test --all-features`.

## 11. Dependency Management & Ecosystem Security

- **Minimal Dependencies:** Keep dependencies to an absolute bare minimum. Do not add external crates for things that can be done with standard Rust or `egui`.
- **Feature Gating:** Any non-trivial dependency (like fonts or flex layouts) must be feature-gated.

## 12. Cross-Platform Developer Experience (DX)

The developer experience must be completely frictionless. Ensure that examples run seamlessly across platforms without hardcoded OS-specific paths.

## 13. Directory Map & Restricted Zones

- `src/` — Pure Rust component code, traits, and theme definitions.
- `docs/adrs/` — Architecture Decision Records (ADRs).
- `examples/` — Showcases demonstrating component usage.
- `assets/` — Resources (fonts, icons) used by features.
- **Restricted:** Do not modify `Cargo.lock` unless intentionally updating or adding a dependency.

## 14. Documentation Standards

- **Rustdoc:** Document **how** to use components (with examples) and **why** a particular sizing or visual workaround was used internally.
- We already maintain detailed manual architecture documentation in the `docs/adrs/` ADRs. Refer to them for system-wide context.
- **Public API:** All public structs, enums, and functions must be documented.

## 15. Common Commands

Use these to validate your work before submitting changes:

- **Check all features:** `cargo check --all-features`
- **Backend validation:** `cargo fmt --all -- --check` && `cargo clippy --all-features -- -D warnings`
- **Run standard showcase:** `cargo run --example showcase_noflex`
- **Run flex showcase:** `cargo run --example showcase --features flex`
