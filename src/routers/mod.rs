use salvo::prelude::*;

use crate::handlers;

pub fn init_router() -> Router {
    let router = Router::new()
        .path("GET /").get(handlers::index)
        .path("GET /api/health").get(handlers::health::health_check)
        .path("GET /api/users").get(handlers::user::list_users)
        .path("POST /api/users").post(handlers::user::create_user)
        .path("GET /api/users/<id>").get(handlers::user::get_user)
        .path("DELETE /api/users/<id>").delete(handlers::user::delete_user);

    router
}