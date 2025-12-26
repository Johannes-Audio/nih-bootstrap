use crate::platform;
use crate::platform::Platform;
use anyhow::Result;
use console::style;

pub struct Dependency {
    pub name: &'static str,
    pub info: &'static str,
}

pub async fn check_and_install(install: bool) -> Result<()> {
    let platform = platform::current();

    println!(
        "{} Checking dependencies for {}.",
        style("===>").green(),
        platform.name()
    );

    let missing = platform.check_dependencies().await?;

    if missing.is_empty() {
        println!(
            "{} All required dependencies are installed.",
            style("===>").green()
        );
        return Ok(());
    }

    println!("\n{} Missing dependencies:", style("===>").yellow());
    for dep in &missing {
        println!("  - {} - {}", dep.name, dep.info);
    }

    if install {
        println!(
            "\n{} Installing missing dependencies...",
            style("===>").green()
        );
        platform.install_dependencies(&missing).await?;
        println!("{} Installation complete.", style("===>").green());
    } else {
        println!(
            "\n{} To install missing dependencies:",
            style("===>").yellow()
        );
        println!("{}", platform.install_instructions(&missing));
        println!("\nOr run: nih-bootstrap deps --install to install them automatically.");
    }

    Ok(())
}
