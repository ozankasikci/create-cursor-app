use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs;

pub mod processor;
pub mod config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
    pub config: config::TemplateConfig,
}

impl Template {
    pub fn new(name: &str, description: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            path,
            config: config::TemplateConfig::default(),
        }
    }

    pub fn load_all() -> Vec<Template> {
        let templates_dir = PathBuf::from("templates");
        let mut templates = Vec::new();

        if let Ok(entries) = fs::read_dir(&templates_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                        templates.push(Template::new(
                            name,
                            "Template description",
                            entry.path(),
                        ));
                    }
                }
            }
        }

        templates
    }
} 