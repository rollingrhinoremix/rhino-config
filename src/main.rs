mod cli;
mod commands;

use std::env::var;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::result::Result;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::commands::{disable, enable};

/// Ask the user a question.
///
/// Asks the user a question, and returns `true` or `false` depending upon if
/// they answered *yes* or *no*.
///
/// # Arguments
///
/// * `message` - A string slice message to display to the user.
/// * `default_yes` - A bool indicating the default answer to the question.
///
/// # Examples
///
/// ```
/// // Example with default of *yes*.
/// ask("Do you want to continue?", true);
///
/// // Example with default of *no*.
/// ask("Do you want to destroy your computer?", false);
/// ```
fn ask(message: &str, default_yes: bool) -> bool {
    print!(
        "{} {} ",
        message,
        if default_yes { "[Y/n]" } else { "[y/N]" }
    );
    io::stdout().flush().unwrap();

    let mut reply = String::new();

    io::stdin()
        .read_line(&mut reply)
        .expect("Error while reading user input.");

    let reply = reply.trim().to_uppercase();

    matches!(reply.as_ref(), "Y" | "")
}

/// Macro to make [`ask`](fn@ask)'s `default_yes` argument a default argument of
/// true.
///
/// Always use this macro instead of the [`ask`](fn@ask) function directly, as
/// it enables you to skip providing the `default_yes` parameter to the
/// [`ask`](fn@ask) function when not needed.
///
/// # Arguments
///
/// * `message` - A string slice message to display to the user.
/// * `default_yes` - A bool indicating the default answer to the question.
///
/// # Examples
///
/// ```
/// // Example with default of *yes*.
/// ask!("Do you want to continue?");
///
/// // Example with default of *no*.
/// ask!("Do you want to destroy your computer?", false);
/// ```
macro_rules! ask {
    ($message:expr) => {
        ask($message, true)
    };
    ($message:expr, $default_yes:expr) => {
        ask($message, $default_yes)
    };
}

fn main() -> Result<(), u8> {
    let cli = Cli::parse();

    let home_dir = var("HOME").expect("Unable to find HOME environment variable!");

    let config_dir = format!("{}/.rhino/config/", home_dir);
    let config_path = Path::new(&config_dir);
    fs::create_dir_all(config_path).expect("Failed to create config directory!");

    let pacstall_config_path = config_path.join("pacstall");
    let mainline_config_path = config_path.join("mainline");
    let snapdpurge_config_path = config_path.join("snapdpurge");

    match &cli.command {
        Commands::Enable(flag) => {
            if flag.interactive {
                if !mainline_config_path.exists() {
                    if ask!("Do you wish to install the Linux mainline kernel?") {
                        enable::mainline(&mainline_config_path);
                    } else {
                        println!(
                            "No changes were made to the Rhino configuration, the mainline kernel \
                             will not be installed."
                        );
                    }
                }

                if !snapdpurge_config_path.exists() {
                    if ask!("Do you wish to remove Snapcraft (snapd) and replace it with Flatpak?")
                    {
                        enable::snapdpurge(&snapdpurge_config_path, &home_dir);
                    } else {
                        println!(
                            "No changes were made to the Rhino configuration, snapd has not been \
                             purged."
                        );
                    }
                }

                if !pacstall_config_path.exists() {
                    if ask!(
                        "Do you wish to enable Pacstall, an additional AUR-like package manager \
                         for Ubuntu on this system?"
                    ) {
                        enable::pacstall(&pacstall_config_path);
                    } else {
                        println!(
                            "No changes were made to the Rhino configuration, Pacstall has not \
                             been enabled."
                        );
                    }
                }
            }

            if flag.mainline {
                if mainline_config_path.exists() {
                    println!("Mainline kernel is already enabled!");
                    return Err(1);
                }
                enable::mainline(&mainline_config_path);
            }

            if flag.snapdpurge {
                if snapdpurge_config_path.exists() {
                    println!("Mainline kernel is already enabled!");
                    return Err(1);
                }
                enable::snapdpurge(&snapdpurge_config_path, &home_dir);
            }

            if flag.pacstall {
                if pacstall_config_path.exists() {
                    println!("Pacstall is already enabled!");
                    return Err(1);
                }
                enable::pacstall(&pacstall_config_path);
            }

            Ok(())
        },
        Commands::Disable(flag) => {
            if flag.mainline {
                if !mainline_config_path.exists() {
                    println!("Mainline kernel is already disabled!");
                    return Err(1);
                }

                disable::mainline(&mainline_config_path);
            }

            if flag.snapdpurge {
                if !snapdpurge_config_path.exists() {
                    println!("Snapdpurge is already disabled!");
                    return Err(1);
                }
                println!("Snapdpurge has been disabled.");

                println!("Reinstalling Snapcraft");

                disable::snapdpurge(&snapdpurge_config_path);
            }

            Ok(())
        },
    }
}
