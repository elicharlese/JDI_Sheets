// file: src/main.rs

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    variants: Vec<Variant>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Variant {
    id: String,
    name: String,
    inventory_quantity: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let query = r#"
        query Products($marketplaceSlug: String!, $productLineSlug: String!) {
          products(marketplaceSlug: $marketplaceSlug, productLineSlug: $productLineSlug) {
            id
            name
            variants {
              id
              name
              inventoryQuantity
            }
          }
        }
    "#;

    let variables = HashMap::from([
        ("marketplaceSlug".to_string(), "shopify".to_string()),
        ("productLineSlug".to_string(), "clothing".to_string()),
    ]);

    let response = client
        .post("http://localhost:8000/graphql")
        .json(&serde_json::json!({ "query": query, "variables": variables }))
        .send()
        .await?;

    let data: HashMap<String, serde_json::Value> = response.json().await?;
    let products: Vec<Product> = serde_json::from_value(data["data"]["products"].clone()).unwrap();

    println!("{:#?}", products);

    Ok(())
}
