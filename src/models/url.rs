use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UrlRequestDTO {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct UrlResponseDTO {
    pub short_code: String,
}

