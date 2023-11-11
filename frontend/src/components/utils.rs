use reqwest::Client;
use yew::{Classes, classes};

/// Function to create the format for the classes! macro
pub fn class(class: &str) -> Classes {
    classes!(class.to_string())
}

/// Simple http get function
pub async fn get(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let text = response.text().await?;
    Ok(text)
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub data: String,
}