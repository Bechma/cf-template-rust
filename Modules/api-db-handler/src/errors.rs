//! Thiserror-backed API error definitions for pokemon.

use http::StatusCode;
use modkit::api::problem::Problem;
use thiserror::Error;

/// Strongly-typed API error codes for RFC 9457 responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum ErrorCode {
    #[error("Pokemon not found")]
    PokemonNotFound,

    #[error("Validation error")]
    PokemonValidation,

    #[error("Internal database error")]
    PokemonInternalDatabase,
}

impl ErrorCode {
    #[must_use]
    pub const fn example2_pokemon_not_found_v1() -> Self {
        Self::PokemonNotFound
    }

    #[must_use]
    pub const fn example2_pokemon_validation_v1() -> Self {
        Self::PokemonValidation
    }

    #[must_use]
    pub const fn example2_pokemon_internal_database_v1() -> Self {
        Self::PokemonInternalDatabase
    }

    #[must_use]
    pub const fn status(self) -> StatusCode {
        match self {
            Self::PokemonNotFound => StatusCode::NOT_FOUND,
            Self::PokemonValidation => StatusCode::UNPROCESSABLE_ENTITY,
            Self::PokemonInternalDatabase => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::PokemonNotFound => "Pokemon Not Found",
            Self::PokemonValidation => "Validation Error",
            Self::PokemonInternalDatabase => "Internal Database Error",
        }
    }

    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::PokemonNotFound => "gts.hx.core.errors.err.v1~hx.example2.pokemon.not_found.v1",
            Self::PokemonValidation => {
                "gts.hx.core.errors.err.v1~hx.example2.pokemon.validation.v1"
            }
            Self::PokemonInternalDatabase => {
                "gts.hx.core.errors.err.v1~hx.example2.pokemon.internal_database.v1"
            }
        }
    }

    #[must_use]
    pub const fn type_url(self) -> &'static str {
        self.code()
    }

    pub fn as_problem(self, detail: impl Into<String>) -> Problem {
        Problem::new(self.status(), self.title(), detail.into())
            .with_code(self.code())
            .with_type(self.type_url())
    }

    pub fn with_context(
        self,
        detail: impl Into<String>,
        instance: &str,
        trace_id: Option<String>,
    ) -> Problem {
        let mut problem = self.as_problem(detail).with_instance(instance);
        if let Some(tid) = trace_id {
            problem = problem.with_trace_id(tid);
        }
        problem
    }
}
