use clap::{App, Arg};
use colored::*;
use crate::lib::templates::{Template, processor::TemplateProcessor};

pub struct CliConfig {
    template: String,
    project_name: String,
    output_dir: std::path::PathBuf,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("create-cursor-app")
        .version("0.1.0")
        .author("Your Name")
        .about("Creates new Cursor IDE projects from templates")
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .value_name("TEMPLATE")
                .help("Template to use (defaults to basic)")
                .default_value("basic")
                .required(false),
        )
        .arg(
            Arg::new("name")
                .help("Project name")
                .required(true),
        )
        .get_matches();

    let config = CliConfig {
        template: matches.value_of("template").unwrap().to_string(),
        project_name: matches.value_of("name").unwrap().to_string(),
        output_dir: std::env::current_dir()?,
    };

    let templates = Template::load_all();
    let template = templates
        .into_iter()
        .find(|t| t.name == config.template)
        .ok_or("Template not found")?;

    let processor = TemplateProcessor::new(
        template,
        config.output_dir.join(&config.project_name),
    );

    processor.process().await?;

    println!("{} Project created successfully!", "Success:".green());
    Ok(())
} 