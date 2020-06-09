use std::io::copy;
use std::fs::File;
use crate::post::{Image, CustomError};
use reqwest::{Client, multipart};
use std::env;
//use serde_json::json;

pub fn build_image() -> Image {
    Image { 
        id: 0,
        file_path: String::from(""),
        url: String::from(""),
    }
}

pub fn get_image(img_src: &String) -> Result<Image, CustomError> {
    let mut image = build_image();
    
    let tmp_dir = "/tmp/nasa/";
    let mut get_resp = reqwest::get(img_src)?;
    debug!("temp dir: '{:?}'", tmp_dir);
    debug!("img_src {}", img_src);
    let mut file = {
        let file_name = get_resp
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        debug!("file to download: '{}'", file_name);
        let file_path = format!("{}{}", tmp_dir, file_name);
        debug!("will be located under: '{:?}'", file_path);
        image.file_path = file_path;
        File::create(&image.file_path)?
    };
    
    debug!("dest File: '{:?}'", file);
    copy(&mut get_resp, &mut file).expect("failed to copy content");
    Ok(image)
}


pub fn post_image(image: Image) -> Result<(), CustomError> {
    let api_url = "https://mstdn.mx/api/v1/media?access_token=";
    let token = env::var("TOKEN").expect("Access token not set");
    let dest = format!("{}{}", api_url, token);
    debug!("dest url: '{}'", dest);

    let form = multipart::Form::new()
        .text("description", "dummy desc")
        .file("file", image.file_path)?;

    let client = Client::new();
    let resp = client
        .post(&dest)
        .multipart(form);

    let text = resp.send()?.text()?;
    debug!("Response is {}", text);

    Ok(())

}