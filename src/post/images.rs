use std::io::copy;
use std::fs::File;
//use tempfile::Builder;

pub enum CustomError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}

pub fn get_image() -> Result<(), CustomError> {
    //let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let tmp_dir = "/tmp/nasa/";
    let img_src = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
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
        let file_name = format!("{}{}", tmp_dir, file_name);
        debug!("will be located under: '{:?}'", file_name);
        File::create(file_name)?
    };
    debug!("dest File: '{:?}'", file);
    copy(&mut get_resp, &mut file).expect("failed to copy content");
    Ok(())
}