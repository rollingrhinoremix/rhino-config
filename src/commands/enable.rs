use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use indoc::indoc;

pub fn mainline(config_path: &Path) -> Result<()> {
    File::create(config_path).context("Failed to create the mainline config file!")?;
    println!(
        r#"The mainline kernel has been enabled - please run "rhino-update" to update your system."#
    );
    Ok(())
}

pub fn pacstall(config_path: &Path) -> Result<()> {
    File::create(config_path).context("Unable to create pacstall config!")?;

    println!(indoc!(
        r#"
        Pacstall has been enabled on the system, please check the
        pacstall documentation on our website for information on how to
        use this utility - please run "rhino-update" to update your system.
        "#
    ));
    Ok(())
}

pub fn snapdpurge(config_path: &Path, home_dir: &str) -> Result<()> {
    File::create(config_path).context("Failed to create the snapdpurge config!")?;

    Command::new("sudo")
        .args(["rm", "-rf", "/var/cache/snapd/"])
        .spawn()
        .context("Failed to remove snapd cache!")?;

    Command::new("sudo")
        .args([
            "apt",
            "autopurge",
            "snapd",
            "gnome-software-plugin-snap",
            "-y",
        ])
        .spawn()
        .context("Failed to remove snapd cache!")?;

    fs::remove_dir_all(Path::new(&format!("{}/snap", home_dir)))
        .context("Failed to remove snap directory!")?;

    Command::new("sudo")
        .args([
            "apt",
            "install",
            "flatpak",
            "gnome-software-plugin-flatpak",
            "-y",
        ])
        .spawn()
        .context("Failed to install flatpak!")?;

    Command::new("flatpak")
        .args([
            "remote-add",
            "--if-not-exists",
            "flathub",
            "https://flathub.org/repo/flathub.flatpakrepo",
        ])
        .spawn()
        .context("Failed to add flathub repository!")?;

    println!("Configuration updated, snapd has been removed from the system.");
    Ok(())
}
