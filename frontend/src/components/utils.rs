use reqwest;
use std::collections::HashMap;
use log::debug;
use reqwest::header::HeaderMap;
use yew::{classes, Classes};

/// Function to create the format for the classes! macro
pub fn class(class: &str) -> Classes {
    classes!(class.to_string())
}

/// Simple http get function
pub async fn get(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    let text = response.text().await?;
    Ok(text)
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub data: String,
}

pub fn query_parser(loc: &str) -> HashMap<String, String> {
    let query_clean = loc.replace("?", "");
    let query_map = query_clean.split('&');
    let mut ret: HashMap<String, String> = HashMap::new();

    for item in query_map.into_iter() {
        if let Some((key, value)) = item.split_once('=') {
            ret.insert(key.to_string(), value.to_string());
        };
    }

    ret
}

pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
