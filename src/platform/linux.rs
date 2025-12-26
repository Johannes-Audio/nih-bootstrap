use super::Platform;
use crate::commands::dependencies::Dependency;
use anyhow::Result;
use std::process::Command;

pub struct Linux;

impl Platform for Linux {
    fn name(&self) -> &'static str {
        "Linux"
    }

    async fn check_dependencies(&self) -> Result<Vec<Dependency>> {
        let mut missing = Vec::new();

        let deps = vec![
            Dependency {
                name: "libasound2-dev",
                info: "shared library for ALSA applications -- development files",
            },
            Dependency {
                name: "libjack-jackd2-dev",
                info: "JACK Audio Connection Kit (development files)",
            },
            Dependency {
                name: "libx11-dev",
                info: "X11 client-side library (development headers)",
            },
            Dependency {
                name: "libx11-xcb-dev",
                info: "Xlib/XCB interface library (development headers)",
            },
            Dependency {
                name: "libxcb1-dev",
                info: "X C Binding, development files",
            },
            Dependency {
                name: "libxcb-util-dev",
                info: "utility libraries for X C Binding -- atom, aux and event",
            },
            Dependency {
                name: "libxcb-render0-dev",
                info: "X C Binding, render extension, development files",
            },
            Dependency {
                name: "libxcb-shape0-dev",
                info: "X C Binding, shape extension, development files",
            },
            Dependency {
                name: "libxcb-xfixes0-dev",
                info: "X C Binding, xfixes extension, development files",
            },
            Dependency {
                name: "libxcb-icccm4-dev",
                info: "utility libraries for X C Binding -- icccm, development files",
            },
            Dependency {
                name: "libxcb-dri2-0-dev",
                info: "lX C Binding, dri2 extension, development files",
            },
            Dependency {
                name: "mesa-common-dev",
                info: "Developer documentation for Mesa",
            },
            Dependency {
                name: "libgl1-mesa-dev",
                info: "transitional dummy package",
            },
            Dependency {
                name: "libxcursor-dev",
                info: "X cursor management library (development files)",
            },
            Dependency {
                name: "pkg-config",
                info: "manage compile and link flags for libraries (transitional package)",
            },
        ];

        for dep in deps {
            let output = Command::new("dpkg").args(["-l", dep.name]).output();

            if let Ok(output) = output {
                if !output.status.success() {
                    missing.push(dep);
                }
            }
        }

        Ok(missing)
    }

    async fn install_dependencies(&self, dependencies: &[Dependency]) -> anyhow::Result<()> {
        let status = Command::new("sudo").args(["apt-get", "update"]).status()?;

        if !status.success() {
            anyhow::bail!("Failed to update package lists.");
        }

        let status = Command::new("sudo")
            .args(["apt-get", "install", "-y"])
            .args(dependencies.iter().map(|dep| dep.name))
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to install dependencies.");
        }

        Ok(())
    }

    fn install_instructions(&self, dependencies: &[Dependency]) -> String {
        let mut deps = Vec::new();

        for dep in dependencies {
            deps.push(dep.name);
        }

        format!(
            "sudo apt-get update && sudo apt-get install -y {}",
            deps.join(" "),
        )
    }
}
