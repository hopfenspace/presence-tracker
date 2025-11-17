use galvyn::core::GalvynRouter;

mod current_presence;

/// Initialize the routes for the frontend
pub fn initialize_routes() -> GalvynRouter {
    GalvynRouter::new().handler(current_presence::current_presence)
}
