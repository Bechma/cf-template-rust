//! REST API route definitions - OpenAPI and Axum routing.

use crate::api::rest::{dto, handlers};
use crate::module::ConcreteAppServices;
use axum::Router;
use modkit::api::OpenApiRegistry;
use modkit::api::operation_builder::LicenseFeature;
use std::sync::Arc;

mod pokemon;

pub(super) struct License;

impl AsRef<str> for License {
    fn as_ref(&self) -> &'static str {
        "gts.x.core.lic.feat.v1~x.core.global.base.v1"
    }
}

impl LicenseFeature for License {}

/// Register all routes for the pokemon module
pub(crate) fn register_routes(
    mut router: Router,
    openapi: &dyn OpenApiRegistry,
    services: Arc<ConcreteAppServices>,
) -> Router {
    router = pokemon::register_pokemon_routes(router, openapi);

    router = router.layer(axum::Extension(services));

    router
}
