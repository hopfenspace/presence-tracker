use std::sync::OnceLock;

use galvyn::core::GalvynRouter;
use galvyn::core::SchemalessJson;
use galvyn::get;
use galvyn::openapi::OpenAPI;
use galvyn::openapi::OpenapiRouterExt;
use galvyn::openapi::get_openapi_for_page;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::instrument;

pub mod handler_frontend;
pub mod handler_sensors;

pub struct FrontendApi;
pub struct SensorApi;

#[get("/openapi.json")]
#[instrument]
pub async fn get_openapi() -> SchemalessJson<&'static OpenAPI> {
    SchemalessJson(galvyn::openapi::get_openapi())
}
#[get("/frontend.json")]
#[instrument]
pub async fn get_frontend_openapi() -> SchemalessJson<&'static OpenAPI> {
    static CACHE: OnceLock<OpenAPI> = OnceLock::new();
    SchemalessJson(CACHE.get_or_init(|| get_openapi_for_page(FrontendApi)))
}
#[get("/sensors.json")]
#[instrument]
pub async fn get_sensors_openapi() -> SchemalessJson<&'static OpenAPI> {
    static CACHE: OnceLock<OpenAPI> = OnceLock::new();
    SchemalessJson(CACHE.get_or_init(|| get_openapi_for_page(SensorApi)))
}

/// Initialize the routes
pub fn initialize_routes() -> GalvynRouter {
    GalvynRouter::new()
        .nest(
            "/docs",
            GalvynRouter::new()
                .openapi_tag("Openapi")
                .handler(get_openapi)
                .handler(get_frontend_openapi)
                .handler(get_sensors_openapi),
        )
        .nest(
            "/api/v1/frontend",
            handler_frontend::initialize_routes().openapi_page(FrontendApi),
        )
        .nest(
            "/api/v1/sensors",
            handler_sensors::initialize_routes().openapi_page(SensorApi),
        )
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO)))
}
