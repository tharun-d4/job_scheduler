use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailInfo {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}
