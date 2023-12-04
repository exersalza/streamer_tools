use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static!{
    pub static ref URL: Mutex<String> = Mutex::new(String::from("localhost:8080"));
}