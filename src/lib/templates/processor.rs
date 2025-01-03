use std::path::PathBuf;
use std::fs;
use crate::lib::templates::Template;

pub struct TemplateProcessor {
    template: Template,
    dest: PathBuf,
}

impl TemplateProcessor {
    pub fn new(template: Template, dest: PathBuf) -> Self {
        Self { template, dest }
    }

    pub async fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create destination directory
        fs::create_dir_all(&self.dest)?;

        // Copy template files
        self.copy_template_files().await?;

        println!("Template processed successfully!");
        Ok(())
    }

    async fn copy_template_files(&self) -> Result<(), std::io::Error> {
        if self.template.path.exists() {
            copy_dir_all(&self.template.path, &self.dest)?;
        }
        Ok(())
    }
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), std::io::Error> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
} 