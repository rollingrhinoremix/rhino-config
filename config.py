#!/usr/bin/env python3

import sys
from pathlib import Path
from shutil import rmtree
from subprocess import run
from textwrap import dedent
from typing import NoReturn

from typer import Option, Typer

app = Typer(
    name="rhino-config",
    no_args_is_help=True,
    context_settings={"help_option_names": ["-h", "--help"]},
    short_help="Utility to edit rolling rhino remix config.",
    help=dedent(
        """
        The Rhino Configuration script allows for you to customise the
        rhino-update utility and extend its capabilities, such as allowing
        for the installation of unsupported software.

        Please be cautious when using rhino-config, issues can arise from
        some of the settings so please ensure that you know what you are
        doing and have read the documentation.
        """
    ),
)

config_path = Path("~/.rhino/config/").expanduser()
config_path.mkdir(parents=True, exist_ok=True)


def ask(message: str, default_yes: bool = True) -> bool:
    """
    Ask the user a question.
    :param message: The message to ask the user.
    :param default_yes: Whether the default answer should be yes or no.
    :return: Whether the user answered yes.
    """

    reply = input(f"{message} {'[Y/n]' if default_yes else '[y/N]'} ").strip().upper()
    return True if reply in ["Y", ""] else False


def enable_mainline(mainline_config_path: Path) -> None:
    """
    Enable mainline kernel in the config.

    :param mainline_config_path: The path to the mainline config.
    """

    mainline_config_path.touch()
    print(
        "Configuration updated! The mainline kernel will be installed on the next update."
    )


def process_snapdpurge(snapdpurge_config_path: Path) -> None:
    """
    Remove Snapcraft and install flatpak, also create the config file.

    :param snapdpurge_config_path: The path to the snapdpurge config.
    """

    snapdpurge_config_path.touch()

    run(["sudo", "rm", "-rf", "/var/cache/snapd/"])
    run(
        [
            "sudo",
            "apt",
            "autopurge",
            "snapd",
            "gnome-software-plugin-snap",
            "-y",
        ]
    )

    rmtree(Path("~/snap").expanduser(), ignore_errors=True)

    run(["sudo", "apt-mark", "hold", "snapd"])
    run(
        [
            "sudo",
            "apt",
            "install",
            "flatpak",
            "gnome-software-plugin-flatpak",
            "-y",
        ]
    )
    run(
        [
            "flatpak",
            "remote-add",
            "--if-not-exists",
            "flathub",
            "https://flathub.org/repo/flathub.flatpakrepo",
        ]
    )

    print("Configuration updated, snapd has been removed from the system.")


def enable_pacstall(pacstall_config_path: Path) -> None:
    """
    Enables Pacstall in the config.

    :param pacstall_config_path: Path to the pacstall config file.
    """

    pacstall_config_path.touch()

    print(
        dedent(
            """
            Pacstall has been enabled on the system, please check the
            pacstall documentation on our website for information on how to
            use this utility.
            """
        ).strip()
    )


@app.command()
def enable(
    mainline: bool = Option(
        False, "-m", "--mainline", help="Install the latest Linux mainline kernel."
    ),
    snapdpurge: bool = Option(
        False,
        "-s",
        "--snapdpurge",
        help="Remove Snapcraft (snapd) and replace it with Flatpak.",
    ),
    pacstall: bool = Option(
        False,
        "-p",
        "--pacstall",
        help="Enable Pacstall,  an additional AUR-like package manager for Ubuntu.",
    ),
    interactive: bool = Option(False, "-i", "--interactive", help="Run interactively."),
) -> NoReturn:
    """Enable a config option, manually or interactively."""

    exit_code = 0

    mainline_config_path = config_path / "mainline"
    snapdpurge_config_path = config_path / "snapdpurge"
    pacstall_config_path = config_path / "pacstall"

    if interactive:
        if not mainline_config_path.exists():
            if ask("Do you wish to install the latest Linux mainline kernel?"):
                enable_mainline(mainline_config_path)
            else:
                print(
                    "No changes were made to the Rhino configuration, The mainline kernel will not be installed."
                )

        if not snapdpurge_config_path.exists():
            if ask(
                "Do you wish to remove Snapcraft (snapd) and replace it with Flatpaks?"
            ):
                process_snapdpurge(snapdpurge_config_path)
            else:
                print(
                    "No changes were made to the Rhino configuration, snapd has not been purged."
                )

        if not pacstall_config_path.exists():
            if ask(
                "Do you wish to enable Pacstall, an additional AUR-like package manager for Ubuntu on this system?"
            ):
                enable_pacstall(pacstall_config_path)
            else:
                print(
                    "No changes were made to the Rhino configuration, Pacstall has not been enabled."
                )

        sys.exit()

    if mainline:
        if not mainline_config_path.exists():
            enable_mainline(mainline_config_path)
        else:
            print("Mainline kernel is already enabled!", file=sys.stderr)
            exit_code = 1

    if snapdpurge:
        if not snapdpurge_config_path.exists():
            process_snapdpurge(snapdpurge_config_path)
        else:
            print("Snapdpurge is already enabled!", file=sys.stderr)
            exit_code = 1

    if pacstall:
        if not pacstall_config_path.exists():
            enable_pacstall(pacstall_config_path)
        else:
            print("Pacstall is already enabled!", file=sys.stderr)
            exit_code = 1

    sys.exit(exit_code)


@app.command()
def disable(
    mainline: bool = Option(
        False, "-m", "--mainline", help="Install the latest Linux mainline kernel."
    ),
    snapdpurge: bool = Option(
        False,
        "-s",
        "--snapdpurge",
        help="Remove Snapcraft (snapd) and replace it with Flatpak.",
    ),
    pacstall: bool = Option(
        False,
        "-p",
        "--pacstall",
        help="Enable Pacstall,  an additional AUR-like package manager for Ubuntu.",
    ),
) -> None:
    """Disable a config option."""

    exit_code = 0

    if mainline:
        if (mainline_config_path := config_path / "mainline").exists():
            mainline_config_path.unlink()
            print("Mainline kernel has been disabled.")
        else:
            print("Mainline kernel is already disabled!", file=sys.stderr)
            exit_code = 1

    if snapdpurge:
        if (snapdpurge_config_path := config_path / "snapdpurge").exists():
            snapdpurge_config_path.unlink()
            print("Snapdpurge has been disabled.")

            print("Reinstalling Snapcraft")

            run(["sudo", "apt", "install", "snapd", "gnome-software-plugin-snap", "-y"])
            run(["sudo", "apt-mark", "unhold", "snapd"])
        else:
            print("Snapdpurge is already disabled!", file=sys.stderr)
            exit_code = 1

    if pacstall:
        if (pacstall_config_path := config_path / "pacstall").exists():
            pacstall_config_path.unlink()
            print("Pacstall has been disabled.")
        else:
            print("Pacstall is already disabled!", file=sys.stderr)
            exit_code = 1

    sys.exit(exit_code)


def main() -> None:
    app()


if __name__ == "__main__":
    main()
