use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub image_title: String,
    pub image_source_url: String,
    pub explanation: String,
    pub published_date:  String,
}