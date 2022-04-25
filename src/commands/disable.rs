use std::fs;
use std::path::Path;
use std::process::Command;

pub fn mainline(config_path: &Path) {
    fs::remove_file(&config_path).expect("Unable to remove mainline config file!");
    println!("Mainline kernel has been disabled.");
}

pub fn snapdpurge(config_path: &Path) {
    fs::remove_file(&config_path).expect("Unable to remove snapdpurge config file!");
    println!("Snapdpurge has been disabled.");

    println!("Reinstalling Snapcradt");

    Command::new("sudo")
        .args([
            "apt",
            "install",
            "snapd",
            "gnome-software-plugin-snap",
            "-y",
        ])
        .spawn()
        .expect("Unable to reinstall snapd!");

    Command::new("sudo")
        .args(["apt-mark", "unhold", "snapd"])
        .spawn()
        .expect("Unable to unhold snapd!");
}
