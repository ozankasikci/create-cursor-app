use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};
use inquire::{Confirm, InquireError};
use crate::lib::templates::Template;

pub struct TemplateProcessor {
    template: Template,
    pub(crate) dest: PathBuf,
    project_name: String,
}

impl TemplateProcessor {
    pub fn new(template: Template, dest: PathBuf, project_name: String) -> Self {
        Self { template, dest, project_name }
    }

    pub async fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate template structure first
        self.validate_template_structure().await?;

        // Create destination directory if it doesn't exist
        if !self.dest.exists() {
            fs::create_dir_all(&self.dest)?;
        }

        // Copy and process template files
        self.copy_template_files().await?;

        println!("Template processed successfully!");
        Ok(())
    }

    async fn copy_template_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.template.path.exists() {
            self.copy_dir_all(&self.template.path, &self.dest)?;
        }
        Ok(())
    }

    fn copy_dir_all(&self, src: &PathBuf, dst: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let dst_path = dst.join(entry.file_name());
            
            if ty.is_dir() {
                self.copy_dir_all(&entry.path(), &dst_path)?;
            } else {
                if dst_path.exists() {
                    let proceed = if std::env::var("TEST_MODE").is_ok() {
                        true // Skip confirmation in test mode
                    } else {
                        Confirm::new(&format!(
                            "File '{}' already exists. Overwrite?",
                            dst_path.display()
                        ))
                        .with_default(false)
                        .prompt()?
                    };

                    if !proceed {
                        println!("Skipping file: {}", dst_path.display());
                        continue;
                    }
                }
                self.copy_and_process_file(&entry.path(), &dst_path)?;
            }
        }
        Ok(())
    }

    fn copy_and_process_file(&self, src: &PathBuf, dst: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        fs::File::open(src)?.read_to_string(&mut content)?;

        // Replace template variables
        let processed_content = content.replace("{{project_name}}", &self.project_name);

        let mut dst_file = fs::File::create(dst)?;
        dst_file.write_all(processed_content.as_bytes())?;
        Ok(())
    }

    pub async fn validate_template_structure(&self) -> Result<(), Box<dyn std::error::Error>> {
        let required_files = vec![
            "PROMPT.md",
            "CHANGELOG.md",
            "PROJECT_SCOPE.md",
            ".cursorrules"
        ];

        for file in required_files {
            if !self.template.path.join(file).exists() {
                return Err(format!("Required file {} is missing from template", file).into());
            }
        }

        Ok(())
    }

    pub fn get_output_path(&self, filename: &str) -> PathBuf {
        self.dest.join(filename)
    }
} 