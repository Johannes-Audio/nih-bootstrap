use super::Platform;
use crate::commands::dependencies::Dependency;
use anyhow::Result;
use std::process::Command;

pub struct Windows;

impl Platform for Windows {
    fn name(&self) -> &'static str {
        "Windows"
    }

    async fn check_dependencies(&self) -> anyhow::Result<Vec<Dependency>> {
        let missing = Vec::<Dependency>::new();

        Ok(missing)
    }

    async fn install_dependencies(&self, dependencies: &[Dependency]) -> anyhow::Result<()> {
        Ok(())
    }

    fn install_instructions(&self, dependencies: &[Dependency]) -> String {
        "".to_string()
    }
}
