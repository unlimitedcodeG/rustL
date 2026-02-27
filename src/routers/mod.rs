use salvo::prelude::*;
use salvo::oapi::swagger_ui::SwaggerUi;
use salvo::oapi::OpenApi;

use crate::handlers;

pub fn init_router() -> Router {
    let api_router = Router::new()
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
        );

    let doc = OpenApi::new("rustL API", "0.1.0").merge_router(&api_router);

    api_router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}