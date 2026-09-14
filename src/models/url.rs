use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UrlShortenRequestDTO {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct UrlShortenResponseDTO {
    pub short_code: String,
}

#[derive(Debug, Deserialize)]
pub struct UrlRedirectRequestDTO {
    pub short_code: String,
}