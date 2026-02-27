use salvo::prelude::*;

#[handler]
pub async fn list_users(res: &mut Response) {
    res.render(Text::Json("[]"));
}

#[handler]
pub async fn create_user(_req: &mut Request, res: &mut Response) {
    res.render(Text::Json(r#"{"message":"user created"}"#));
}

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let id = match req.param::<String>("id") {
        Some(value) => value,
        None => String::new(),
    };
    res.render(Text::Json(format!(r#"{{"id":"{}"}}"#, id)));
}

#[handler]
pub async fn delete_user(req: &mut Request, res: &mut Response) {
    let id = match req.param::<String>("id") {
        Some(value) => value,
        None => String::new(),
    };
    res.render(Text::Json(format!(r#"{{"message":"user {} deleted"}}"#, id)));
}
