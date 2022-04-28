use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

pub fn mainline(config_path: &Path) -> Result<()> {
    fs::remove_file(&config_path).context("Unable to remove mainline config file!")?;
    println!("Mainline kernel has been disabled.");

    Ok(())
}

pub fn snapdpurge(config_path: &Path) -> Result<()> {
    fs::remove_file(&config_path).context("Unable to remove snapdpurge config file!")?;
    println!("Snapdpurge has been disabled.");

    println!("Reinstalling Snapcraft...");

    Command::new("sudo")
        .args([
            "apt",
            "install",
            "snapd",
            "gnome-software-plugin-snap",
            "-y",
        ])
        .spawn()
        .context("Unable to reinstall snapd!")?;

    Command::new("sudo")
        .args(["apt-mark", "unhold", "snapd"])
        .spawn()
        .context("Unable to unhold snapd!")?;

    Ok(())
}

pub fn pacstall(config_path: &Path) -> Result<()> {
    fs::remove_file(&config_path).context("Unable to remove pacstall config file!")?;
    println!("Pacstall has been disabled.");
    Ok(())
}
