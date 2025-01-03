use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TemplateConfig {
    pub dependencies: Vec<String>,
    pub cursorrules: CursorRules,
    pub post_install: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CursorRules {
    pub language: String,
    pub framework: Option<String>,
    pub settings: Option<serde_json::Value>,
} 