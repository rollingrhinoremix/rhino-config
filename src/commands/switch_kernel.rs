use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result};

pub fn xanmod(config_path: &Path) -> Result<()> {
    File::create(config_path).context("Failed to create the xanmod config file!")?;
    println!(r#"The XanMod kernel has been enabled - please run "rhino-update" to install it."#);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use rstest::*;
    use tempfile::{tempdir, TempDir};

    #[fixture]
    fn temp_dir() -> TempDir { tempdir().unwrap() }

    #[rstest]
    fn test_xanmod(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("xanmod");

        super::xanmod(&config_path)?;
        // Test that the config file is created
        assert!(config_path.exists());
        
        Ok(())
    }
}
