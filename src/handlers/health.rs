use salvo::prelude::*;

#[handler]
pub async fn health_check(res: &mut Response) {
    res.render(Text::Plain("OK"));
}
