use salvo::prelude::*;
use salvo::oapi::ToSchema;
use salvo::oapi::extract::PathParam;
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct MessageBody {
    message: String,
}

#[derive(Serialize, ToSchema)]
struct UserBody {
    id: String,
}

#[endpoint]
pub async fn list_users() -> Json<Vec<String>> {
    Json(Vec::new())
}

#[endpoint]
pub async fn create_user() -> Json<MessageBody> {
    Json(MessageBody {
        message: "user created".to_owned(),
    })
}

#[endpoint]
pub async fn get_user(id: PathParam<String>) -> Json<UserBody> {
    Json(UserBody {
        id: id.into_inner(),
    })
}

#[endpoint]
pub async fn delete_user(id: PathParam<String>) -> Json<MessageBody> {
    Json(MessageBody {
        message: format!("user {} deleted", id.into_inner()),
    })
}
