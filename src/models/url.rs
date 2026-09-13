use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UrlRequestDTO {
    pub url: String,
}

#[derive(Debug, Serialize)] // TODO: Use it!
pub struct UrlResponseDTO {
    pub short_code: String,
}
