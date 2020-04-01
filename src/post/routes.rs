use crate::post::Post;
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

#[post("/posts")]
async fn create(post: web::Json<Post>) -> impl Responder {
    let json_post = post.into_inner();
    info!("{}", json!(json_post));
    HttpResponse::Created().json(json_post)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    //cfg.service(find_all);
    //cfg.service(find);
    //cfg.service(update);
    //cfg.service(delete);
    cfg.service(create);
}