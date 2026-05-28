//! Middleware layers for the API gateway.
//!
//! Currently provides request-ID generation and propagation, which are used by
//! the tracing layer to correlate logs with individual HTTP requests.

pub mod request_id;
