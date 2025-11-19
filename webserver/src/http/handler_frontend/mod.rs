use galvyn::core::GalvynRouter;

mod presence;

/// Initialize the routes for the frontend
pub fn initialize_routes() -> GalvynRouter {
    GalvynRouter::new().handler(presence::data)
}
