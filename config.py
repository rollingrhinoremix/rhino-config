# Imports
import os
from pathlib import Path
from shutil import rmtree
from textwrap import dedent

config_path = Path("~/.rhino/config/").expanduser()
config_path.mkdir(parents=True, exist_ok=True)


def main():
    # Splash screen
    print(
        dedent(
            """
            Welcome to the Rhino configuration script
            ---
            The Rhino Configuration script allows for you to customise the rhino-update utility and extend its capabilities, such as allowing for the installation of unsupported software.
            Please be cautious when using rhino-config, issues can arise from some of the settings so please ensure that you know what you are doing and have read the documentation.
            ---
            """
        )
    )

    # Give the user the choice of installing the latest mainline kernel and take user input
    mainline = input("Do you wish to install the latest Linux mainline kernel? [Y/n] ")

    # If user input is yes, go to the mainline install function
    if mainline == "Y" or mainline == "y" or mainline == "":
        (config_path / "mainline").touch(exist_ok=True)
        print(
            "Configuration updated! The mainline kernel will be installed on the next update"
        )
    # If user input is no, go to the mainline denied function
    elif mainline == "n" or mainline == "N":
        print(
            "No changes were made to the Rhino configuration, The mainline kernel will not be installed."
        )

    print("---")

    snapd = input(
        "Do you wish to remove Snapcraft (snapd) and replace it with Flatpaks? [Y/n]"
    )
    if snapd == "Y" or snapd == "y" or snapd == "":
        (config_path / "snapdpurge").touch(exist_ok=True)
        os.system("sudo rm -rf /var/cache/snapd/")
        os.system("sudo apt autoremove --purge snapd gnome-software-plugin-snap -y")
        rmtree(Path("~/snap").expanduser(), ignore_errors=True)
        os.system("sudo apt-mark hold snapd")
        os.system("sudo apt install flatpak gnome-software-plugin-flatpak -y")
        os.system(
            "flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo"
        )
        print("Configuration updated, snapd has been removed from the system.")
    elif snapd == "n" or snapd == "N":
        print(
            "No changes were made to the Rhino configuration, snapd has not been purged"
        )

    # Print that the program has completed and then exit
    print(
        "\nrhino-config has completed, please run rhino-update to update your system!"
    )
    quit()


if __name__ == "__main__":
    main()
