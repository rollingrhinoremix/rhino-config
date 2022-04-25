use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use clap::CommandFactory;
use clap_complete::{generate_to, shells};
use clap_mangen::Man;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = PathBuf::from(env::var_os("OUT_DIR").ok_or(ErrorKind::NotFound)?);

    let mut cmd = Cli::command();
    let bash_path = generate_to(shells::Bash, &mut cmd, "rhino-config", outdir.clone())?;

    println!(
        "cargo:warning=completion file for bash generated: {:?}",
        bash_path
    );

    let elvish_path = generate_to(shells::Elvish, &mut cmd, "rhino-config", outdir.clone())?;

    println!(
        "cargo:warning=completion file for elvish generated: {:?}",
        elvish_path
    );

    let fish_path = generate_to(shells::Fish, &mut cmd, "rhino-config", outdir.clone())?;

    println!(
        "cargo:warning=completion file for fish generated: {:?}",
        fish_path
    );

    let powershell_path =
        generate_to(shells::PowerShell, &mut cmd, "rhino-config", outdir.clone())?;

    println!(
        "cargo:warning=completion file for powershell generated: {:?}",
        powershell_path
    );

    // let zsh_path = generate_to(
    //     shells::Zsh,
    //     &mut cmd,
    //     "rhino-config",
    //     outdir.clone(),
    // )?;

    // println!("cargo:warning=completion file for zsh generated: {:?}", zsh_path);

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    let man_path = outdir.join("rhino-config.1");
    fs::write(&man_path, buffer)?;

    println!("cargo:warning=man page generated: {:?}", &man_path);

    Ok(())
}
