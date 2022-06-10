mod cli;
mod commands;

use std::env::var;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use anyhow::{ensure, Context, Result};
use clap::Parser;

use crate::cli::{Cli, Commands, DisableKernel, EnableKernel, SwitchKernel};
use crate::commands::{disable, enable, switch_kernel};

/// Ask the user a question.
///
/// Asks the user a question, and returns `true` or `false` depending upon if
/// they answered *yes* or *no*.
///
/// # Arguments
///
/// * `message` - A string slice message to display to the user.
///
/// # Examples
///
/// ```
/// ask("Do you want to continue?"); 
/// ```
fn ask(message: &str) -> bool {
    print!("{} [Y/n] ", message);
    io::stdout().flush().unwrap();

    let mut reply = String::new();

    io::stdin()
        .read_line(&mut reply)
        .expect("Error while reading user input.");

    let reply = reply.trim().to_uppercase();

    matches!(reply.as_ref(), "Y" | "")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let home_dir = var("HOME").context("Unable to find HOME environment variable!")?;
    let home_path = Path::new(&home_dir);

    let config_path = home_path.join(".rhino/config/");
    fs::create_dir_all(&config_path).context("Failed to create config directory!")?;

    let pacstall_config_path = config_path.join("pacstall");
    let mainline_config_path = config_path.join("mainline");

    let snapdpurge_config_path = config_path.join("snapdpurge");
    let snapdpurge_snap_path = home_path.join("snap/");

    match &cli.command {
        Commands::Enable(flag) => {
            if flag.interactive {
                if !mainline_config_path.exists() {
                    if ask("Do you wish to install the Linux mainline kernel?") {
                        enable::mainline(&mainline_config_path)?;
                    } else {
                        println!(
                            "No changes were made to the Rhino configuration, the mainline kernel \
                             will not be installed."
                        );
                    }
                }

                if !snapdpurge_config_path.exists() {
                    if ask("Do you wish to remove Snapcraft (snapd) and replace it with Flatpak?") {
                        enable::snapdpurge(&snapdpurge_config_path, &snapdpurge_snap_path)?;
                    } else {
                        println!(
                            "No changes were made to the Rhino configuration, snapd has not been \
                             purged."
                        );
                    }
                }

                if !pacstall_config_path.exists() {
                    if ask(
                        "Do you wish to enable Pacstall, an additional AUR-like package manager \
                         for Ubuntu on this system?",
                    ) {
                        enable::pacstall(&pacstall_config_path)?;
                    } else {
                        println!(
                            "No changes were made to the Rhino configuration, Pacstall has not \
                             been enabled."
                        );
                    }
                }
            }

            if flag.mainline {
                ensure!(
                    !mainline_config_path.exists(),
                    "Mainine kernel is already enabled!"
                );
                enable::mainline(&mainline_config_path)?;
            }

            if flag.snapdpurge {
                ensure!(
                    !snapdpurge_config_path.exists(),
                    "Snapdpurge is already enabled!"
                );
                enable::snapdpurge(&snapdpurge_config_path, &snapdpurge_snap_path)?;
            }

            if flag.pacstall {
                ensure!(
                    !pacstall_config_path.exists(),
                    "Pacstall is already enabled!"
                );
                enable::pacstall(&pacstall_config_path)?;
            }

            Ok(())
        },
        Commands::Disable(flag) => {
            if flag.mainline {
                ensure!(
                    mainline_config_path.exists(),
                    "Mainline kernel is already disabled!"
                );

                disable::mainline(&mainline_config_path)?;
            }

            if flag.snapdpurge {
                ensure!(
                    snapdpurge_config_path.exists(),
                    "Snapdpurge is already disabled!"
                );

                disable::snapdpurge(&snapdpurge_config_path)?;
            }

            if flag.pacstall {
                ensure!(
                    pacstall_config_path.exists(),
                    "Pacstall is already disabled!"
                );

                disable::pacstall(&pacstall_config_path)?;
            }

            Ok(())
        },

        // Commands::SwitchKernel(kernel) => match &kernel {

        // },
        Commands::SwitchKernel(operation) => {
            let xanmod_config_path = &config_path.join("xanmod");
            let liquorix_config_path = &config_path.join("liquorix");

            match operation {
                SwitchKernel::Enable(kernel) => match kernel {
                    EnableKernel::Xanmod => switch_kernel::enable_xanmod(xanmod_config_path),
                    EnableKernel::Liquorix => switch_kernel::enable_liquorix(liquorix_config_path),
                },

                SwitchKernel::Disable(kernel) => match kernel {
                    DisableKernel::Xanmod => switch_kernel::disable_xanmod(xanmod_config_path),
                    DisableKernel::Liquorix => {
                        switch_kernel::disable_liquorix(liquorix_config_path)
                    },
                },
            }
        },
    }
}
