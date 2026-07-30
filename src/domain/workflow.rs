use super::{category::Category, risk::Risk, step::Step, tool::Tool};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub tool: Tool,
    pub category: Category,
    pub risk: Risk,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub steps: Vec<Step>,
}
