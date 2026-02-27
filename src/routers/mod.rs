use salvo::prelude::*;

use crate::handlers;

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("").get(handlers::index))
        .push(Router::with_path("api/health").get(handlers::health::health_check))
        .push(
            Router::with_path("api/users")
                .get(handlers::user::list_users)
                .post(handlers::user::create_user),
        )
        .push(
            Router::with_path("api/users/<id>")
                .get(handlers::user::get_user)
                .delete(handlers::user::delete_user),
        )
}