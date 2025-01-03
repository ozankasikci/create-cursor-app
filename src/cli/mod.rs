use clap::{App, Arg};
use console::style;
use inquire::{required, Text};
use crate::lib::templates::{Template, processor::TemplateProcessor};

pub struct CliConfig {
    template: String,
    project_name: String,
    output_dir: std::path::PathBuf,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("🚀 Create Cursor App").cyan().bold());
    println!("{}\n", style("Interactive project generator").dim());

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
            Arg::new("directory")
                .help("Directory to create the project in")
                .index(1)
                .required(true),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();

    // Always prompt for project name
    let project_name = Text::new("What's your project name?")
        .with_help_message("The name will be used in template files")
        .with_validator(required!("Project name is required"))
        .with_placeholder("my-awesome-project")
        .prompt()?;

    let config = CliConfig {
        template: matches.value_of("template").unwrap().to_string(),
        project_name: project_name.clone(),
        output_dir: std::env::current_dir()?,
    };

    let templates = Template::load_all();
    let template = templates
        .into_iter()
        .find(|t| t.name == config.template)
        .ok_or("Template not found")?;

    println!("\n{}", style("⚙️  Creating project...").dim());

    let processor = TemplateProcessor::new(
        template,
        config.output_dir.join(directory), // Use the provided directory
        project_name,
    );

    processor.process().await?;

    println!("\n{} Project {} created successfully!", 
        style("✨"),
        style(&config.project_name).cyan().bold()
    );
    
    println!("\n{}", style("To get started:").dim());
    println!("  cd {}", style(directory).cyan());
    println!("  {}", style("Happy coding! 🎉").green());
    
    Ok(())
} 