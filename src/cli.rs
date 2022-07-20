use clap::{ArgGroup, Args, Parser, Subcommand};

/// Utility to edit rolling rhino remix config
///
/// The Rhino Configuration script allows for you to customise the
/// rhino-update utility and extend its capabilities, such as allowing
/// for the installation of unsupported software
///
/// Please be cautious when using rhino-config, issues can arise from
/// some of the settings so please ensure that you know what you are
/// doing and have read the documentation
#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Enable a config option, manually or interactively
    ///
    /// This will enable rhino-update to download, download, install and keep a
    /// specific option updated, such as the latest Linux kernel
    Enable(EnableCommand),

    /// Disable a config option
    ///
    /// Disable options you've enabled through this utility
    Disable(DisableCommand),

    #[clap(subcommand)]
    /// Switch to alternate kernels
    SwitchKernel(SwitchKernel),
}

#[derive(Args)]
#[clap(group(
    ArgGroup::new("operations")
        .multiple(true)
        .args(&["pacstall", "snapdpurge"]),
))]
pub struct EnableCommand {
    /// Run interactively
    ///
    /// Interactive mode steps through each configuration one-by-one so that
    /// users with less experience with the command-line are able to easily
    /// utilise rhino-config
    #[clap(conflicts_with("operations"))]
    #[clap(required_unless_present("operations"))]
    #[clap(short, long)]
    pub interactive: bool,

    /// Enable Pacstall, an additional AUR-like package manager for Ubuntu
    ///
    /// Pacstall can be enabled via the rhino-config enable command. This will
    /// download, install and keep Pacstall updated. Pacstall is an AUR-like
    /// package manager for Ubuntu and Ubuntu-based systems
    #[clap(short, long)]
    pub pacstall: bool,

    /// Remove Snapcraft (snapd) and replace it with Flatpak
    ///
    /// This will be removing snapd from your system. It
    /// will hold snapd from being reinstalled via apt as well. It will utilise
    /// Flatpak as a drop-in replacement and will automatically install Flatpak
    /// as well as the Flathub repositories
    #[clap(short, long)]
    pub snapdpurge: bool,
}

#[derive(Args)]
#[clap(group(
            ArgGroup::new("operations")
                .required(true)
                .multiple(true)
                .args(&["pacstall", "snapdpurge"]),
        ))]
pub struct DisableCommand {
    /// Disable Pacstall, an additional AUR-like package manager for Ubuntu
    ///
    /// This disables and uninstalls Pacstall from your system. Pacstall will
    /// no longer be updated and all applications installed via Pacstall will
    /// have to be manually updated or removed from your system
    #[clap(short, long)]
    pub pacstall: bool,

    /// Install Snapcraft (snapd)
    ///
    /// This reinstalls snapd on your system. Flatpak will still remain on your
    /// system and will have to be manually removed
    #[clap(short, long)]
    pub snapdpurge: bool,
}
#[derive(Subcommand)]
pub enum SwitchKernel {
    /// Enable custom kernels
    #[clap(subcommand)]
    Enable(EnableKernel),

    /// Disable custom kernels
    #[clap(subcommand)]
    Disable(DisableKernel),
}

#[derive(Subcommand)]
pub enum EnableKernel {
    /// Enable the XanMod kernel. Defaults to the stable variant
    ///
    /// XanMod is a general-purpose Linux kernel distribution with custom
    /// settings and new features. Built to provide a stable, responsive and
    /// smooth desktop experience
    Xanmod(XanmodVariants),

    /// Enable the Liquorix kernel
    ///
    /// Liquorix is a distro kernel replacement built using the best
    /// configuration and kernel sources for desktop, multimedia, and gaming
    /// workloads
    Liquorix,

    /// Enable the Libre kernel
    ///
    /// Linux-libre is a modified version of the Linux kernel that contains no
    /// binary blobs, obfuscated code, or code released under proprietary
    /// licenses
    Libre,
}

#[derive(Subcommand)]
pub enum DisableKernel {
    /// Disable the XanMod kernel. Defaults to the stable variant
    Xanmod(XanmodVariants),

    /// Disable the Liquorix kernel
    Liquorix,

    /// Disable the Libre kernel
    Libre,
}

#[derive(Args, PartialEq, Default)]
pub struct XanmodVariants {
    /// Stable variant
    #[clap(short, long)]
    pub stable: bool,

    /// Mainline variant
    #[clap(short, long)]
    pub edge: bool,

    /// Long-Term-Support variant
    #[clap(short, long)]
    pub lts: bool,

    /// Realtime variant
    #[clap(short, long)]
    pub realtime: bool,

    /// Uses the Task Type CPU scheduler (in development)
    #[clap(long)]
    pub tasktype: bool,

    /// Realtime variant of the edge kernel (in development)
    #[clap(long)]
    pub realtime_edge: bool,
}
