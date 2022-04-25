use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use indoc::indoc;

pub fn mainline(config_path: &Path) {
    File::create(config_path).expect("Failed to create the mainline config file!");
    println!("Configuration updated! The mainline kernel would be installed on the next update.");
}

pub fn pacstall(config_path: &Path) {
    File::create(config_path).expect("Unable to create pacstall config!");

    println!(indoc!(
        "
        Pacstall has been enabled on the system, please check the
        pacstall documentation on our website for information on how to
        use this utility.
        "
    ));
}

pub fn snapdpurge(config_path: &Path, home_dir: &str) {
    File::create(config_path).expect("Failed to create the snapdpurge config!");

    Command::new("sudo")
        .args(["rm", "-rf", "/var/cache/snapd/"])
        .spawn()
        .expect("Failed to remove snapd cache!");

    Command::new("sudo")
        .args([
            "apt",
            "autopurge",
            "snapd",
            "gnome-software-plugin-snap",
            "-y",
        ])
        .spawn()
        .expect("Failed to remove snapd cache!");

    fs::remove_dir_all(Path::new(&format!("{}/snap", home_dir)))
        .expect("Failed to remove snap directory!");

    Command::new("sudo")
        .args([
            "apt",
            "install",
            "flatpak",
            "gnome-software-plugin-flatpak",
            "-y",
        ])
        .spawn()
        .expect("Failed to install flatpak!");

    Command::new("flatpak")
        .args([
            "remote-add",
            "--if-not-exists",
            "flathub",
            "https://flathub.org/repo/flathub.flatpakrepo",
        ])
        .spawn()
        .expect("Failed to add flathub repository!");

    println!("Configuration updated, snapd has been removed from the system.");
}
