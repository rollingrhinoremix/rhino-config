use std::fs::{self, File};
use std::path::Path;

use anyhow::{ensure, Context, Result};

use crate::cli::XanmodVariants;

macro_rules! create_config {
    ($name: ident, $config_dir: ident) => {{
        let config_path = $config_dir.join(concat!("xanmod-", stringify!($name)));
        ensure!(
            !config_path.exists(),
            format!(r#"The XanMod kernel variant: `{}` is already enabled! Run "rhino-update" to install it."#, stringify!($name))
        );
        File::create(config_path).context(format!("Failed to create the xanmod-{} config file!", stringify!($name)))?;
        println!("{}", format!(r#"The XanMod kernel variant: `{}` is enabled! Run "rhino-update" to install it."#, stringify!($name)));

    }};
    ($name: ident, $config_dir: ident, $variants: ident) => {{
        if $variants.$name {
            create_config!($name, $config_dir);
        }
    }}
}

macro_rules! remove_config {
    ($name:ident, $config_dir:ident) => {{
        let config_path = $config_dir.join(concat!("xanmod-", stringify!($name)));
        ensure!(
            config_path.exists(),
            format!(
                r#"The XanMod kernel variant: `{}` is already disabled!"#,
                stringify!($name)
            )
        );
        fs::remove_file(config_path).context(format!(
            "Failed to remove the xanmod-{} config file!",
            stringify!($name)
        ))?;
        println!(
            "{}",
            format!(
                r#"The XanMod kernel variant: `{}` has been disabled!"#,
                stringify!($name)
            )
        );
    }};
    ($name:ident, $config_dir:ident, $variants:ident) => {{
        if $variants.$name {
            remove_config!($name, $config_dir);
        }
    }};
}

pub fn enable_xanmod(config_dir: &Path, variants: &XanmodVariants) -> Result<()> {
    create_config!(stable, config_dir, variants);
    create_config!(edge, config_dir, variants);
    create_config!(lts, config_dir, variants);
    create_config!(realtime, config_dir, variants);
    create_config!(realtime_edge, config_dir, variants);
    create_config!(tasktype, config_dir, variants);

    // Default to stable variant if no other variants are specfied
    if variants == &XanmodVariants::default() {
        create_config!(stable, config_dir);
    }
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

pub fn enable_libre(config_path: &Path) -> Result<()> {
    ensure!(
        !config_path.exists(),
        r#"The Libre kernel is already enabled! Run "rhino-update" to install it."#
    );
    File::create(config_path).context("Failed to create the libre config file!")?;
    println!(r#"The Libre kernel has been enabled - please run "rhino-update" to install it."#);
    Ok(())
}

pub fn disable_xanmod(config_dir: &Path, variants: &XanmodVariants) -> Result<()> {
    remove_config!(stable, config_dir, variants);
    remove_config!(edge, config_dir, variants);
    remove_config!(lts, config_dir, variants);
    remove_config!(realtime, config_dir, variants);
    remove_config!(realtime_edge, config_dir, variants);
    remove_config!(tasktype, config_dir, variants);

    // Default to stable variant if no other variants are specfied
    if variants == &XanmodVariants::default() {
        remove_config!(stable, config_dir);
    }
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

pub fn disable_libre(config_path: &Path) -> Result<()> {
    ensure!(
        config_path.exists(),
        r#"The Libre kernel is already disabled!"#
    );
    fs::remove_file(config_path).context("Failed to remove the libre config file!")?;
    println!(r#"The Libre kernel has been disabled."#);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::File;

    use rstest::*;
    use tempfile::{tempdir, TempDir};

    use crate::cli::XanmodVariants;

    #[fixture]
    fn temp_dir() -> TempDir { tempdir().unwrap() }

    #[rstest]
    fn test_enable_xanmod(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_dir = temp_dir.path();
        let variants = XanmodVariants {
            stable: true,
            edge: true,
            lts: true,
            realtime: true,
            realtime_edge: true,
            tasktype: true,
        };

        // Test that the config file is created
        super::enable_xanmod(&config_dir, &variants)?;
        assert!(config_dir.join("xanmod-stable").exists());
        assert!(config_dir.join("xanmod-edge").exists());
        assert!(config_dir.join("xanmod-lts").exists());
        assert!(config_dir.join("xanmod-realtime").exists());
        assert!(config_dir.join("xanmod-realtime_edge").exists());
        assert!(config_dir.join("xanmod-tasktype").exists());

        // Test that it errors out if the config file is already present
        assert_eq!(
            super::enable_xanmod(&config_dir, &variants)
                .unwrap_err()
                .to_string(),
            r#"The XanMod kernel variant: `stable` is already enabled! Run "rhino-update" to install it."#
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
    fn test_enable_libre(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("libre");

        // Test that the config file is created
        super::enable_libre(&config_path)?;
        assert!(config_path.exists());

        // Test that it errors out if the config file is already present
        assert_eq!(
            super::enable_libre(&config_path)
                .unwrap_err()
                .to_string(),
            r#"The Libre kernel is already enabled! Run "rhino-update" to install it."#
        );

        Ok(())
    }

    #[rstest]
    fn test_disable_xanmod(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_dir = temp_dir.path();
        let variants = XanmodVariants {
            stable: true,
            edge: true,
            lts: true,
            realtime: true,
            realtime_edge: true,
            tasktype: true,
        };

        let xanmod_stable = &config_dir.join("xanmod-stable");
        let xanmod_edge = &config_dir.join("xanmod-edge");
        let xanmod_lts = &config_dir.join("xanmod-lts");
        let xanmod_realtime = &config_dir.join("xanmod-realtime");
        let xanmod_realtime_edge = &config_dir.join("xanmod-realtime_edge");
        let xanmod_tasktype = &config_dir.join("xanmod-tasktype");

        File::create(xanmod_stable)?;
        File::create(xanmod_edge)?;
        File::create(xanmod_lts)?;
        File::create(xanmod_realtime)?;
        File::create(xanmod_realtime_edge)?;
        File::create(xanmod_tasktype)?;

        // Test that the config file is deleted
        super::disable_xanmod(&config_dir, &variants)?;
        assert!(!xanmod_stable.exists());
        assert!(!xanmod_edge.exists());
        assert!(!xanmod_lts.exists());
        assert!(!xanmod_realtime.exists());
        assert!(!xanmod_realtime_edge.exists());
        assert!(!xanmod_tasktype.exists());

        // Test that it errors out if the config file is not present
        assert_eq!(
            super::disable_xanmod(&config_dir, &variants)
                .unwrap_err()
                .to_string(),
            r#"The XanMod kernel variant: `stable` is already disabled!"#
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

    #[rstest]
    fn test_disable_libre(temp_dir: TempDir) -> Result<(), Box<dyn Error>> {
        let config_path = temp_dir.path().join("libre");
        File::create(&config_path)?;

        // Test that the config file is deleted
        super::disable_libre(&config_path)?;
        assert!(!config_path.exists());

        // Test that it errors out if the config file is not present
        assert_eq!(
            super::disable_libre(&config_path)
                .unwrap_err()
                .to_string(),
            r#"The Libre kernel is already disabled!"#
        );
        Ok(())
    }
}
