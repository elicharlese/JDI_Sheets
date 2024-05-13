#![allow(dead_code)]
#![allow(clippy::unit_arg)]

use reqwest::{Client, Response};
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::utils::constants::{EBAY_API_TOKEN};

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    #[serde(rename = "asin")]
    pub asin: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "price")]
    pub price: f64,
    #[serde(rename = "link")]
    pub link: String,
    #[serde(rename = "image")]
    pub image: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResponse {
    pub products: Vec<Product>,
}

pub async fn search_products(query: &str) -> Result<Vec<Product>, Box<dyn std::error::Error>> {
    let client = Client::new();
    // let url = format!("https://api.ebay.com/buy/v1/products/search?q={}", query);
    let url = "https://api.ebay.com/buy/v1/products/search?q=shirt";
    let response = client
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", std::env::var("EBAY_API_TOKEN")?),
        )
        .send()
        .await?;

    let search_response: SearchResponse = response.json().await?;

    Ok(search_response.products)
}

