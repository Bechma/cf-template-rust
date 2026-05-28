#![doc = include_str!("../README.md")]

// ── Module definition (aggregate root) ──────────────────────────────────────
mod module;
pub use module::{{ project-name | pascal_case }};

// ── Internal sub-modules ─────────────────────────────────────────────────────
mod config;
pub mod middleware;
mod web;

// ── Public re-exports ────────────────────────────────────────────────────────
pub use config::{{ project-name | pascal_case }}Config;
