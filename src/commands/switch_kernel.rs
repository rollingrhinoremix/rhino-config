use std::fs::{self, File};
use std::path::Path;

use anyhow::{ensure, Context, Result};

pub fn enable_xanmod(config_path: &Path) -> Result<()> {
    ensure!(
        !config_path.exists(),
        r#"The XanMod kernel is already enabled! Run "rhino-update" to install it."#
    );
    File::create(config_path).context("Failed to create the xanmod config file!")?;
    println!(r#"The XanMod kernel has been enabled - please run "rhino-update" to install it."#);
    Ok(())
}

pub fn enable_liquorix(config_path: &Path) -> Result<()> {
    ensure!(
        !config_path.exists(),
        r#"The Liquorix kernel is already enabled! Run "rhino-update" to install it."#
    );
    File::create(config_path).context("Failed to create the liquorix config file!")?;
    println!(r#"The Liquorix kernel has been enabled - please run "rhino-update" to install it."#);
    Ok(())
}

pub fn disable_xanmod(config_path: &Path) -> Result<()> {
    ensure!(
        config_path.exists(),
        r#"The XanMod kernel is already disabled!"#
    );
    fs::remove_file(config_path).context("Failed to remove the xanmod config file!")?;
    println!(r#"The XanMod kernel has been disabled."#);
    Ok(())
}

pub fn disable_liquorix(config_path: &Path) -> Result<()> {
    ensure!(
        config_path.exists(),
        r#"The Liquorix kernel is already disabled!"#
    );
    fs::remove_file(config_path).context("Failed to remove the liquorix config file!")?;
    println!(r#"The Liquorix kernel has been disabled."#);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::File;

    use rstest::*;
    use tempfile::{tempdir, TempDir};

    #[fixture]
    fn temp_dir() -> TempDir { tempdir().unwrap() }

    #[rstest]
    fn test_enable_xanmod(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("xanmod");

        // Test that the config file is created
        super::enable_xanmod(&config_path)?;
        assert!(config_path.exists());

        // Test that it errors out if the config file is already present
        assert_eq!(
            super::enable_xanmod(&config_path).unwrap_err().to_string(),
            r#"The XanMod kernel is already enabled! Run "rhino-update" to install it."#
        );

        Ok(())
    }

    #[rstest]
    fn test_enable_liquorix(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("liquorix");

        // Test that the config file is created
        super::enable_liquorix(&config_path)?;
        assert!(config_path.exists());

        // Test that it errors out if the config file is already present
        assert_eq!(
            super::enable_liquorix(&config_path)
                .unwrap_err()
                .to_string(),
            r#"The Liquorix kernel is already enabled! Run "rhino-update" to install it."#
        );

        Ok(())
    }

    #[rstest]
    fn test_disable_xanmod(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("xanmod");
        File::create(&config_path)?;

        // Test that the config file is deleted
        super::disable_xanmod(&config_path)?;
        assert!(!config_path.exists());

        // Test that it errors out if the config file is not present
        assert_eq!(
            super::disable_xanmod(&config_path).unwrap_err().to_string(),
            r#"The XanMod kernel is already disabled!"#
        );
        Ok(())
    }

    #[rstest]
    fn test_disable_liquorix(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("liquorix");
        File::create(&config_path)?;

        // Test that the config file is deleted
        super::disable_liquorix(&config_path)?;
        assert!(!config_path.exists());

        // Test that it errors out if the config file is not present
        assert_eq!(
            super::disable_liquorix(&config_path)
                .unwrap_err()
                .to_string(),
            r#"The Liquorix kernel is already disabled!"#
        );
        Ok(())
    }
}
