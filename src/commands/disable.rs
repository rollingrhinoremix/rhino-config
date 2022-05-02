use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{ensure, Context, Result};

pub fn mainline(config_path: &Path) -> Result<()> {
    fs::remove_file(&config_path).context("Unable to remove mainline config file!")?;
    println!("Mainline kernel has been disabled.");

    Ok(())
}
pub fn pacstall(config_path: &Path) -> Result<()> {
    fs::remove_file(&config_path).context("Unable to remove pacstall config file!")?;
    println!("Pacstall has been disabled.");
    println!("Removing pacstall...");

    // Get the uninstall script from `curl` or `wget` depending upon which is
    // installed on the system. Capture the output also.
    let uninstall_script = if Command::new("curl").output()?.status.success() {
        String::from_utf8(
            Command::new("curl")
                .args(["-fsSL", "https://git.io/JEZbi"])
                .output()?
                .stdout,
        )?
    } else {
        String::from_utf8(
            Command::new("wget")
                .args(["-q", "https://git.io/JEZbi", "-O", "-"])
                .output()?
                .stdout,
        )?
    };
    ensure!(Command::new("bash")
        .args(["-c", &uninstall_script])
        .status()?
        .success());
    Ok(())
}

pub fn snapdpurge(config_path: &Path) -> Result<()> {
    fs::remove_file(&config_path).context("Unable to remove snapdpurge config file!")?;
    println!("Snapdpurge has been disabled.");

    println!("Reinstalling Snapcraft...");

    ensure!(Command::new("sudo")
        .args([
            "apt",
            "install",
            "snapd",
            "gnome-software-plugin-snap",
            "-y",
        ])
        .status()
        .context("Unable to reinstall snapd!")?
        .success());

    ensure!(Command::new("sudo")
        .args(["apt-mark", "unhold", "snapd"])
        .status()
        .context("Unable to unhold snapd!")?
        .success());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env::var;
    use std::error::Error;
    use std::fs::File;
    use std::process::Command;

    use rstest::*;
    use tempfile::{tempdir, TempDir};

    #[fixture]
    fn temp_dir() -> TempDir { tempdir().unwrap() }

    #[rstest]
    fn test_mainline(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("mainline");
        File::create(&config_path)?;

        super::mainline(&config_path)?;
        // Test that the config file is deleted
        assert!(!config_path.exists());

        Ok(())
    }

    #[rstest]
    fn test_pacstall(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("pacstall");
        File::create(&config_path)?;

        super::pacstall(&config_path)?;
        // Test that the config file is deleted
        assert!(!config_path.exists());

        Ok(())
    }

    #[rstest]
    fn test_snapdpurge(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("snapdpurge");
        File::create(&config_path)?;
        let snapd_previously_installed = Command::new("dpkg")
            .args(["--status", "snapd"])
            .status()?
            .success();

        super::snapdpurge(&config_path)?;
        // Test that the config file is deleted
        assert!(!config_path.exists());
        // Test that `snapd` and `gnome-software-plugin-snap` have been installed
        assert!(Command::new("dpkg")
            .args(["--status", "snapd", "gnome-software-plugin-snap"])
            .status()?
            .success());
        // Test that `snapd` is unholded, i.e, it doesn't appear on `apt-mark showhold`
        assert!(!String::from_utf8(
            Command::new("sh")
                .arg("apt-mark")
                .arg("showhold")
                .output()?
                .stdout
        )?
        .contains("snapd"));

        // Purge `snapd` and `gnome-software-plugin-snap` if previously not installed
        // before test Don't run if the test is being run on a CI
        if !snapd_previously_installed && var("CI").is_err() {
            Command::new("sudo")
                .args([
                    "apt",
                    "autopurge",
                    "snapd",
                    "gnome-software-plugin-snap",
                    "--assume-yes",
                ])
                .status()?;
        }
        Ok(())
    }
}
