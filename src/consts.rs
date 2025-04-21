use foyer::{Cache, CacheBuilder};

pub const SWAGGER_UI: &'static [u8] = include_bytes!("../swagger/swagger.html");
pub const SWAGGER_BUNDLE: &'static [u8] = include_bytes!("../swagger/swagger-ui-bundle.js");
pub const SWAGGER_ES_BUNDLE: &'static [u8] = include_bytes!("../swagger/swagger-ui-es-bundle.js");
pub const SWAGGER_STANDALONE: &'static [u8] =
    include_bytes!("../swagger/swagger-ui-standalone-preset.js");
pub const SWAGGER_CSS: &'static [u8] = include_bytes!("../swagger/swagger-ui.css");
pub const SWAGGER_JS: &'static [u8] = include_bytes!("../swagger/swagger-ui.js");