use super::{command::Command, risk::Risk};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub command: Command,
    pub risk: Risk,
}
