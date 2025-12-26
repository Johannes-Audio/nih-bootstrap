#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::Linux as CurrentPlatform;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacOS as CurrentPlatform;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::Windows as CurrentPlatform;

use crate::commands::dependencies::Dependency;

pub trait Platform {
    fn name(&self) -> &'static str;

    async fn check_dependencies(&self) -> anyhow::Result<Vec<Dependency>>;

    async fn install_dependencies(&self, dependencies: &[Dependency]) -> anyhow::Result<()>;

    fn install_instructions(&self, dependencies: &[Dependency]) -> String;
}

pub fn current() -> CurrentPlatform {
    CurrentPlatform
}
