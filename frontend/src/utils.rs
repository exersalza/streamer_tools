use yew::{Classes, classes};

pub fn class(class: &str) -> Classes {
    classes!(class.to_string())
}