use tracing::info;

use crate::api::rest::dto::PokemonDto;

use modkit::api::prelude::*;
use modkit::api::select::{apply_select, page_to_projected_json};

use modkit_security::SecurityContext;

mod pokemon;

// ==================== Pokemon Handlers ====================

pub(crate) use pokemon::get_pokemon;
pub(crate) use pokemon::list_pokemon;
