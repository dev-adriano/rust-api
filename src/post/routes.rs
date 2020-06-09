use crate::post::{NasaImage, get_image, post_image};
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

#[post("/posts")]
async fn create(nasa_image: web::Json<NasaImage>) -> impl Responder {
    info!("Image Src: {}", nasa_image.image_source_url);
   
    match get_image(&nasa_image.image_source_url) {
        Err(_error) => {            
            error!("An error was occured while downloading the image. ");
        },
        Ok(img) => {
            debug!("Donwload image competed. {}", img.file_path);
            let _ = post_image(img);
        }
    }
    

    let src_json = nasa_image.into_inner();
    info!("{}", json!(src_json));
    HttpResponse::Created().json(src_json)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    //cfg.service(find_all);
    //cfg.service(find);
    //cfg.service(update);
    //cfg.service(delete);
    cfg.service(create);
}