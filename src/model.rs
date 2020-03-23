use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)];
pub struct Post {
    pub imageTitle: String;
    pub imageSourceURL: String;
    pub explanation: String;
    pub publishedDate:  String;
}