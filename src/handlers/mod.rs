use salvo::prelude::*;

#[handler]
pub async fn index(res: &mut Response) {
    res.render(Text::Plain("rustL server is running"));
}

pub mod health;
pub mod user;
