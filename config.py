#!/usr/bin/env python3

import sys
from pathlib import Path
from shutil import rmtree
from subprocess import run
from textwrap import dedent
from typing import NoReturn


def ask(message: str, default_yes: bool = True) -> bool:
    """
    Ask the user a question.

    :param message: The message to ask the user.
    :param default_yes: Whether the default answer should be yes or no.
    :return: Whether the user answered yes.
    """

    reply = input(f"{message} {'[Y/n]' if default_yes else '[y/N]'} ").strip().upper()
    return True if reply in ["Y", ""] else False


def main() -> NoReturn:
    config_path = Path("~/.rhino/config/").expanduser()
    config_path.mkdir(parents=True, exist_ok=True)

    # Splash screen
    print(
        dedent(
            """
            Welcome to the Rhino configuration script.
            ---
            The Rhino Configuration script allows for you to customise the
            rhino-update utility and extend its capabilities, such as allowing
            for the installation of unsupported software.

            Please be cautious when using rhino-config, issues can arise from
            some of the settings so please ensure that you know what you are
            doing and have read the documentation.
            ---
            """
        )
    )

    if not (mainline_config_path := config_path / "mainline").exists():
        if ask("Do you wish to install the latest Linux mainline kernel?"):
            mainline_config_path.touch()
            print(
                "Configuration updated! The mainline kernel will be installed on the next update."
            )
        else:
            print(
                "No changes were made to the Rhino configuration, The mainline kernel will not be installed."
            )

        print("---")

    if not (snapd_purge_config_path := config_path / "snapdpurge").exists():
        if ask("Do you wish to remove Snapcraft (snapd) and replace it with Flatpaks?"):
            snapd_purge_config_path.touch()

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
        else:
            print(
                "No changes were made to the Rhino configuration, snapd has not been purged."
            )

    if not (pacstall_config_path := config_path / "pacstall").exists():
        if ask(
            "Do you wish to enable Pacstall, an additional AUR-like package manager for Ubuntu on this system?"
        ):
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

        else:
            print(
                "No changes were made to the Rhino configuration, Pacstall has not been enabled."
            )

    # Print that the program has completed and then exit
    print(
        "\nrhino-config has completed, please run rhino-update to update your system!"
    )
    sys.exit()


if __name__ == "__main__":
    main()
