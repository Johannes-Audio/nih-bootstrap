use crate::config::Config;
use crate::templates::{TemplateContext, TemplateRenderer};
use anyhow::Result;
use console::style;
use std::fs;
use std::path::Path;

pub async fn run(
    name: &str,
    path: &str,
    description: &str,
    init_git: bool,
    init_ci: bool,
    gui: &str,
) -> Result<()> {
    println!("{} Creating new plugin: {}.", style("===>").green(), name);

    let config = Config::load()?;
    let renderer = TemplateRenderer::new(gui);

    let underscored = name.replace("-", "_");
    let camelcase = to_camelcase(&underscored);

    let target_path = Path::new(path).join(name);
    if target_path.exists() {
        anyhow::bail!("Directory '{}' already exists!", target_path.display());
    }

    fs::create_dir_all(&target_path)?;

    println!("{} Creating project structure...", style("===>").green());

    renderer.render_project(
        &target_path,
        &TemplateContext {
            project_name: name,
            underscored_name: &underscored,
            camelcase_name: &camelcase,
            description,
            config: &config,
        },
    )?;

    println!(
        "{} Project files created at '{}'.",
        style("===>").green(),
        target_path.display()
    );

    if init_git {
        if !crate::commands::git::is_git_installed() {
            eprintln!(
                "{} Warining: Git is not installed. Skipping Git initialization.",
                style("===>").yellow()
            );
            eprintln!("{} Install Git and run 'git init'.", style("===>").yellow());
        } else {
            match crate::commands::git::init_repo(&target_path, init_ci, name).await {
                Ok(_) => println!("{} Git repository initialized.", style("===>").green()),
                Err(e) => {
                    eprintln!(
                        "{} Warning: Failed to initialize git: {}.",
                        style("===>").yellow(),
                        e
                    );
                    eprintln!(
                        "{} You can manually run 'git init'.",
                        style("===>").yellow()
                    );
                }
            }
        }
    }

    show_next_steps(
        &target_path,
        name,
        init_git && crate::commands::git::is_git_installed(),
    );

    Ok(())
}

fn to_camelcase(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
            continue;
        }

        if !capitalize_next {
            result.push(c);
        } else {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        }
    }

    result
}

fn show_next_steps(path: &Path, name: &str, git_initialized: bool) {
    println!("\n{} Next steps:", style("===>").green());
    println!("  cd {}", path.display());

    if !git_initialized {
        println!("  git init");
        println!("  git add .");
        println!("  git commit -m 'Initial commit for {}'", name);
    }

    println!(
        "\n{} To build project into a VST3 plugin:",
        style("===>").green()
    );
    println!("  cargo xtask bundle {} --release", name);
    println!("\n{} To check dependencies: ", style("===>").green());
    println!("  nih-bootstrap deps");

    if crate::commands::git::is_git_installed() && !git_initialized {
        println!("{} To add git and CI/CD later:", style("===>").yellow());
        println!("  nih-bootstrap git --ci");
    }

    println!("\nHappy coding! 🚀");
}
