
use axum::{body, Json};
use serde_json::json;

use chrono::Utc;
use rand::random_range;
use sqids::Sqids;

fn get_epoch() -> String {
    Utc::now().timestamp().to_string()
}

pub fn create_sqid_token() -> String{
    let digits: Vec<u32> = get_epoch()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let random_size = random_range(6..=9) as u8;
    let digits_u64: Vec<u64> = digits.iter().map(|&d| d as u64).collect();
    let sqids = Sqids::builder()
        .alphabet("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect())
        .min_length(random_size.clone())
        .build()
        .unwrap(); // Handle error as needed
    let token = sqids.encode(&digits_u64).unwrap_or_default();
    token.chars().take(random_size as usize).collect()
}

pub async fn create_short_url(
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, axum::response::Response> {
    let url = body.get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| axum::response::Response::builder()
            .status(400)
            .body(body::Body::from("Invalid URL"))
            .unwrap())?;

    let url_token = create_sqid_token();
    Ok(
        Json(
            json!(
                {
                    "shortUrl": url_token,
                    "originalUrl": url
                }
            )
        )
    )
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_epoch() {
        let epoch = get_epoch();
        assert!(!epoch.is_empty(), "epoch should not be empty");
    }
}