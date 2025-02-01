use std::fmt::Display;

use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub enum ButtonFunction {
    M5,
    M1,
    Stop,
    Play,
    Pause,
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
            "Pause" => ButtonFunction::Pause,
            "P1" => ButtonFunction::P1,
            "P5" => ButtonFunction::P5,
            _ => ButtonFunction::Stop, // Default case
        }
    }
}
