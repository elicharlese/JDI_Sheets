use reqwest::{Client, Response};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "name")]
    pub name: String,
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
    let url = format!("https://makersplace.michaels.com/api/v1/items?q={}", query);
    let response = client
        .get(url)
        .send()
        .await?;

    let search_response: SearchResponse = response.json().await?;

    Ok(search_response.products)
}

