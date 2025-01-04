use create_cursor_app::lib::templates::{Template, processor::TemplateProcessor};
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

async fn create_test_template(dir: &PathBuf) -> Result<(), std::io::Error> {
    // Create required files with proper content structure
    fs::write(
        dir.join("PROMPT.md"),
        r#"# AI Assistant Instructions

## Critical Context Files
ALWAYS check and analyze these files before any action:
- CHANGELOG.md - Contains project history and documentation protocols
- PROJECT_SCOPE.md - Contains project goals, architecture, and boundaries

## Project: {{project_name}}"#,
    )?;

    fs::write(
        dir.join("CHANGELOG.md"),
        r#"# Changelog

## Project: {{project_name}}

### [Unreleased]
- Initial project setup
- Basic template structure"#,
    )?;

    fs::write(
        dir.join("PROJECT_SCOPE.md"),
        r#"# {{project_name}} - Project Scope

## Project Overview
[Project description]

## Core Objectives
1. [Objective 1]
2. [Objective 2]"#,
    )?;

    fs::write(
        dir.join(".cursorrules"),
        r#"{
    "settings": {
        "ai_behavior": {
            "pre_action_check": "PROMPT.md",
            "documentation_required": true,
            "strict_mode": true
        },
        "context_tracking": {
            "enabled": true,
            "initialization": {
                "required_files": ["PROMPT.md", "CHANGELOG.md", "PROJECT_SCOPE.md"],
                "order": ["PROMPT.md", "CHANGELOG.md", "PROJECT_SCOPE.md"]
            },
            "files": ["PROMPT.md", "CHANGELOG.md", "PROJECT_SCOPE.md"]
        }
    }
}"#,
    )?;

    Ok(())
}

#[tokio::test]
async fn test_template_processing() {
    let temp_dir = TempDir::new().unwrap();
    let template_dir = temp_dir.path().join("template");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir(&template_dir).unwrap();
    create_test_template(&template_dir).await.unwrap();

    let template = Template::new(
        "test",
        "Test template",
        template_dir,
    );

    let processor = TemplateProcessor::new(
        template,
        output_dir,
        "Test Project".to_string(),
    );

    processor.process().await.unwrap();

    let processed_content = fs::read_to_string(
        processor.get_output_path("PROMPT.md")
    ).unwrap();

    assert!(processed_content.contains("Project: Test Project"));
}

#[tokio::test]
async fn test_variable_replacement() {
    let temp_dir = TempDir::new().unwrap();
    let template_dir = temp_dir.path().join("template");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir(&template_dir).unwrap();
    create_test_template(&template_dir).await.unwrap();

    fs::write(
        template_dir.join("README.md"),
        "# {{project_name}}\n## About {{project_name}}",
    ).unwrap();

    let template = Template::new(
        "test",
        "Test template",
        template_dir,
    );

    let processor = TemplateProcessor::new(
        template,
        output_dir,
        "My App".to_string(),
    );

    processor.process().await.unwrap();

    let processed_content = fs::read_to_string(
        processor.get_output_path("README.md")
    ).unwrap();

    assert_eq!(processed_content, "# My App\n## About My App");
}

#[tokio::test]
async fn test_template_structure_validation() {
    let temp_dir = TempDir::new().unwrap();
    let template_dir = temp_dir.path().join("template");
    fs::create_dir(&template_dir).unwrap();

    fs::write(
        template_dir.join("README.md"),
        "# {{project_name}}",
    ).unwrap();

    let template = Template::new(
        "test",
        "Test template",
        template_dir,
    );

    let processor = TemplateProcessor::new(
        template,
        temp_dir.path().join("output"),
        "Test Project".to_string(),
    );

    let result = processor.validate_template_structure().await;
    assert!(result.is_err());
} 

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_template_loading() {
        // Create a temporary templates directory
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("basic");
        fs::create_dir(&template_path).unwrap();

        // Create some template files
        fs::write(
            template_path.join("PROMPT.md"),
            "# {{project_name}}",
        ).unwrap();

        let template = Template::new(
            "basic",
            "Basic template",
            template_path,
        );

        assert_eq!(template.name, "basic");
        assert_eq!(template.description, "Basic template");
    }

    #[test]
    fn test_load_all_templates() {
        // Create a temporary templates directory
        let temp_dir = TempDir::new().unwrap();
        let templates_dir = temp_dir.path().join("templates");
        fs::create_dir(&templates_dir).unwrap();

        // Create basic template
        let basic_path = templates_dir.join("basic");
        fs::create_dir(&basic_path).unwrap();

        // Create typescript template
        let ts_path = templates_dir.join("typescript");
        fs::create_dir(&ts_path).unwrap();

        let templates = Template::load_from_dir(&templates_dir);
        assert_eq!(templates.len(), 2);
    }
} 