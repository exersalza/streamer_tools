use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::Deserialize;

use parking_lot::Mutex;
use std::sync::Arc;

lazy_static! {
    pub static ref event_bus: Arc<Mutex<Events>> = Arc::new(Mutex::new(Events::new()));
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub enum ButtonFunction {
    M5,
    M1,
    Stop,
    Play,
    P1,
    P5,
}

impl From<String> for ButtonFunction {
    fn from(s: String) -> Self {
        match s.as_str() {
            "M5" => ButtonFunction::M5,
            "M1" => ButtonFunction::M1,
            "Stop" => ButtonFunction::Stop,
            "Play" => ButtonFunction::Play,
            "P1" => ButtonFunction::P1,
            "P5" => ButtonFunction::P5,
            _ => ButtonFunction::Stop,
        }
    }
}

type CBType = fn(Option<HashMap<String, String>>);

#[derive(PartialEq, Eq, Hash)]
pub enum EventTypes {
    Update = 0,
}

// ZnxTech: good combo
pub struct Events {
    __callbacks: HashMap<EventTypes, Vec<CBType>>,
}

impl Events {
    pub fn new() -> Self {
        Self {
            __callbacks: HashMap::new(),
        }
    }

    pub fn add_callback(&mut self, event: EventTypes, cb: CBType) -> anyhow::Result<()> {
        if let Some(val) = self.__callbacks.get_mut(&event) {
            val.push(cb);
            return Ok(());
        };
        self.__callbacks.insert(event, vec![cb]);
        Ok(())
    }

    pub fn remove_callback(&mut self, event: EventTypes, cb: CBType) {
        if let Some(val) = self.__callbacks.get_mut(&event) {
            val.retain(|v| !std::ptr::fn_addr_eq(*v, cb));
        }
    }

    pub fn trigger_event(&self, event: EventTypes, data: Option<HashMap<String, String>>) {
        if let Some(val) = self.__callbacks.get(&event) {
            val.iter().for_each(|func| func(data.clone()));
        }
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}
