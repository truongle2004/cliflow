use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub namespace: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub example: String,
    pub command: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub danger: Danger,
    #[serde(default)]
    pub args: Vec<Arg>,
}

impl Recipe {
    pub fn key(&self) -> String {
        format!("{}/{}", self.namespace, self.id)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Arg {
    pub name: String,
    pub prompt: String,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Danger {
    Low,
    Medium,
    High,
}

impl fmt::Display for Danger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
