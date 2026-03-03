//! API DB Handler - Pokemon Module
//!
//! Pokemon management with REST API, database storage, and inter-module
//! communication via `ClientHub`.
//!
//! ## Architecture
//!
//! This crate follows clean architecture with strict layering:
//!
//! ### SDK Crate (`api-db-handler-sdk`)
//! - Public API contract for inter-module communication
//! - `PokemonClientV1` trait, `Pokemon` model, `PokemonError`, OData schemas
//!
//! ### API Layer (`crate::api`)
//! - HTTP/REST interface: routes, handlers, DTOs, error mapping
//!
//! ### Domain Layer (`crate::domain`)
//! - Business logic, repository traits, domain errors
//!
//! ### Infrastructure Layer (`crate::infra`)
//! - Database persistence, SeaORM entities, migrations, OData mapping

// === API ERROR DEFINITIONS ===
pub mod errors;

// === MODULE DEFINITION ===
pub mod module;
pub use module::PokemonModule;

// === INTERNAL MODULES ===
#[doc(hidden)]
pub mod api;
#[doc(hidden)]
pub mod config;
#[doc(hidden)]
pub mod domain;
#[doc(hidden)]
pub mod infra;
