use salvo::prelude::*;

#[endpoint]
pub async fn health_check() -> &'static str {
    "OK"
}
