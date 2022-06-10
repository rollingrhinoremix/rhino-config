use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use anyhow::{ensure, Context, Result};
use indoc::indoc;

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

pub fn snapdpurge(config_path: &Path, snap_path: &Path) -> Result<()> {
    File::create(config_path).context("Failed to create the snapdpurge config!")?;

    ensure!(Command::new("sudo")
        .args(["rm", "-rf", "/var/cache/snapd/"])
        .status()
        .context("Failed to remove snapd cache!")?
        .success());

    ensure!(Command::new("sudo")
        .args([
            "apt",
            "autopurge",
            "snapd",
            "gnome-software-plugin-snap",
            "-y",
        ])
        .status()
        .context("Failed to remove snapd cache!")?
        .success());

    fs::remove_dir_all(&snap_path).context("Failed to remove snap directory!")?;

    ensure!(Command::new("sudo")
        .args([
            "apt",
            "install",
            "flatpak",
            "gnome-software-plugin-flatpak",
            "-y",
        ])
        .status()
        .context("Failed to install flatpak!")?
        .success());

    ensure!(Command::new("sudo")
        .args([
            "flatpak",
            "remote-add",
            "--if-not-exists",
            "flathub",
            "https://flathub.org/repo/flathub.flatpakrepo",
        ])
        .status()
        .context("Failed to add flathub repository!")?
        .success());

    println!("Configuration updated, snapd has been removed from the system.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::process::Command;

    use rstest::*;
    use tempfile::{tempdir, TempDir};

    #[fixture]
    fn temp_dir() -> TempDir { tempdir().unwrap() }

    #[rstest]
    fn test_pacstall(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("pacstall");

        super::pacstall(&config_path)?;
        // Test that the config file is created
        assert!(config_path.exists());

        Ok(())
    }

    #[rstest]
    fn test_snapdpurge(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("snapdpurge");
        let snap_dir = tempdir().unwrap();
        let snap_path = snap_dir.path();

        super::snapdpurge(&config_path, snap_path)?;
        // Test that the config file is created
        assert!(config_path.exists());
        // Test that the snap_path has been deleted
        assert!(!snap_path.exists());
        // Test that `snapd` and `gnome-software-plugin-snap` have been uninstalled
        assert!(!Command::new("dpkg")
            .args(["--status", "snapd", "gnome-software-plugin-snap"])
            .status()?
            .success());
        // Test that `flatpak` has been installed
        assert!(!Command::new("dpkg")
            .args(["--status", "flatpak", "gnome-software-plugin-flatpak"])
            .status()?
            .success());
        // Test that the flathub repository has been added to flatpak
        let output = Command::new("flatpak").args(["remotes"]).output()?.stdout;
        assert!(String::from_utf8(output)?.contains("flathub"));

        Ok(())
    }
}
