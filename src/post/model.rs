use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NasaImage {
    pub image_title: String,
    pub image_source_url: String,
    pub explanation: String,
    pub published_date:  String,
}

pub struct Image {
    pub id: u64,
    pub file_path: String,
    pub url: String,
}

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