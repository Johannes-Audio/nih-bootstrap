use crate::config::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct TemplateContext<'a> {
    pub project_name: &'a str,
    pub underscored_name: &'a str,
    pub camelcase_name: &'a str,
    pub description: &'a str,
    pub config: &'a Config,
}

pub struct TemplateRenderer {
    gui_framework: String,
}

impl TemplateRenderer {
    pub fn new(gui_framework: &str) -> Self {
        Self {
            gui_framework: gui_framework.to_string(),
        }
    }

    pub fn render_project(&self, target_path: &Path, context: &TemplateContext) -> Result<()> {
        let project_dir = target_path.join(context.underscored_name);
        fs::create_dir_all(&project_dir.join("src"))?;

        // create 3 main project files: main.rs, lib.rs and editor.rs
        self.render_file(
            &project_dir.join("src/main.rs"),
            include_str!("../../templates/project/main.txt"),
            context,
        )?;

        self.render_file(
            &project_dir.join("src/lib.rs"),
            self.get_lib_template(),
            context,
        )?;

        self.render_file(
            &project_dir.join("src/editor.rs"),
            self.get_editor_template(),
            context,
        )?;

        self.render_file(
            &project_dir.join("Cargo.toml"),
            include_str!("../../templates/project/cargo_project.txt"),
            context,
        )?;

        self.create_xtask_project(target_path, context)?;

        Ok(())
    }

    fn get_lib_template(&self) -> &'static str {
        match self.gui_framework.as_str() {
            "iced" => include_str!("../../templates/project/lib_iced.txt"),
            _ => include_str!("../../templates/project/lib_iced.txt"),
        }
    }

    fn get_editor_template(&self) -> &'static str {
        match self.gui_framework.as_str() {
            "iced" => include_str!("../../templates/project/editor.txt"),
            _ => include_str!("../../templates/project/editor.txt"),
        }
    }

    fn render_file(&self, path: &Path, template: &str, context: &TemplateContext) -> Result<()> {
        let content = template
            .replace("%%PROJECT_NAME%%", context.project_name)
            .replace("%%PROJECT_NAME_UNDERSCORED%%", context.underscored_name)
            .replace("%%PROJECT_NAME_CAMELCASE%%", context.camelcase_name)
            .replace("%%PROJECT_DESCRIPTION%%", context.description)
            .replace("%%AUTHORS%%", &context.config.authors)
            .replace(
                "%%CARGO_PACKAGE_VERSION%%",
                &context.config.cargo_pkg_version,
            )
            .replace("%%VENDOR%%", &context.config.vendor)
            .replace("%%URL%%", &context.config.vendor_url)
            .replace("%%EMAIL%%", &context.config.vendor_email)
            .replace("%%NIH_PLUG_GIT%%", &context.config.nih_plug_git);

        fs::write(path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;

        Ok(())
    }

    fn create_xtask_project(&self, target_path: &Path, context: &TemplateContext) -> Result<()> {
        let xtask_dir = target_path.join("xtask");
        fs::create_dir_all(&xtask_dir.join("src"))?;

        // xtask files
        fs::write(
            xtask_dir.join("src/main.rs"),
            include_str!("../../templates/xtask/main.rs"),
        )?;

        self.render_file(
            &xtask_dir.join("Cargo.toml"),
            include_str!("../../templates/xtask/cargo_xtask.txt"),
            context,
        )?;

        // workspace Cargo.toml
        self.render_file(
            &target_path.join("Cargo.toml"),
            include_str!("../../templates/xtask/cargo_workspace.txt"),
            context,
        )?;

        // xtask alias
        let cargo_config_dir = target_path.join(".cargo");

        fs::create_dir_all(&cargo_config_dir)?;
        fs::write(
            cargo_config_dir.join("config.toml"),
            include_str!("../../templates/xtask/cargo_config.toml"),
        )?;

        Ok(())
    }
}
