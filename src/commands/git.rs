use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;
use std::process::Command;

pub async fn init_repo(project_path: &Path, init_ci: bool, project_name: &str) -> Result<()> {
    println!("{} Setting up Git repository...", style("===>").green());

    // git repository initialization
    let output = Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(project_path)
        .output()
        .context("Failed to run git init.")?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to initialize git repository. Git output: {}",
            String::from_utf8_lossy(&output.stderr),
        );
    }

    // .gitignore file
    let gitignore_content = include_str!("../..//templates/git/gitignore.txt");

    fs::write(project_path.join(".gitignore"), gitignore_content)
        .context("Failed to create .gitignore file.")?;

    if init_ci {
        setup_ci(project_path, project_name)?;
    }

    // stage files
    let output = Command::new("git")
        .args(["add", "."])
        .current_dir(project_path)
        .output()
        .context("Failed to run git add.")?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to stage files. Git output: {}",
            String::from_utf8_lossy(&output.stderr),
        );
    }

    // initial commit
    let output = Command::new("git")
        .args([
            "commit",
            "-m",
            format!("Initial commit for {}", project_name).as_str(),
        ])
        .current_dir(project_path)
        .output()
        .context("Failed to run git commit.")?;

    if !output.status.success() {
        eprintln!(
            "{} Warning: Failed to create initial commit.",
            style("===>").yellow()
        );
        eprintln!(
            "{} You may need to configure git user.name and user.email.",
            style("===>").yellow()
        );
    } else {
        println!(
            "{} Git repository initialized with initial commit.",
            style("===>").green()
        );
    }

    Ok(())
}

pub async fn setup(init_ci: bool) -> Result<()> {
    println!("{} Setting up Git...", style("===>").green());

    // check if this we are in an existing git repository
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .context("Failed to check git status")?;

    if !output.status.success() {
        anyhow::bail!("Not in a git repository. Run 'git init' first.");
    }

    // get current directory as project name for CI
    let current_dir = std::env::current_dir()?;
    let project_name = current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project");

    // ensure .gitignore exists
    let gitignore_path = Path::new(".gitignore");
    if !gitignore_path.exists() {
        let gitignore_content = include_str!("../../templates/git/gitignore.txt");
        fs::write(gitignore_path, gitignore_content).context("Failed to create .gitignore")?;
        println!("{} Created .gitignore file.", style("===>").green());
    }

    if init_ci {
        setup_ci(&current_dir, project_name)?;
    }

    println!("{} Git setup complete.", style("===>").green());

    if !init_ci {
        println!("\n{} To add CI/CD workflows later:", style("===>").yellow());
        println!("  nih-bootstrap git --ci");
    }

    Ok(())
}

fn setup_ci(project_path: &Path, project_name: &str) -> Result<()> {
    println!("{} Setting up CI/CD workflows...", style("===>").green());

    let workflows_dir = project_path.join(".github").join("workflows");
    fs::create_dir_all(&workflows_dir).context("Failed to create .github/workflows directory.")?;

    let ci_content = include_str!("../../templates/git/ci_cd_general.yaml");
    fs::write(workflows_dir.join("general.yaml"), ci_content)
        .context("Failed to create CI/CD workflow file.")?;

    println!(
        "{} Created CI/CD workflow for project {}.",
        style("===>").green(),
        project_name
    );

    Ok(())
}

pub fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
