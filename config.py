# Imports
import os

# Change configuration to enable the mainline kernel
def mainlineInstall():
    os.system("touch ~/.rhino/config/mainline")
    print(
        "Configuration updated! The mainline kernel will be installed on the next update"
    )


# Do not enable the mainline kernel
def mainlineDenied():
    print(
        "No changes were made to the Rhino configuration, The mainline kernel will not be installed."
    )


# Purge snapd from system and mark it as hold on apt so it will not be reinstalled
def snapdPurge():
    os.system("touch ~/.rhino/config/snapdpurge")
    os.system("sudo rm -rf /var/cache/snapd/")
    os.system("sudo apt autoremove --purge snapd gnome-software-plugin-snap -y")
    os.system("rm -fr ~/snap")
    os.system("sudo apt-mark hold snapd")
    os.system("sudo apt install flatpak gnome-software-plugin-flatpak -y")
    os.system(
        "flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo"
    )
    print("Configuration updated, snapd has been removed from the system.")


# Do not purge snapd from system
def snapdKeep():
    print("No changes were made to the Rhino configuration, snapd has not been purged")


# Main program
def config():
    # Splash screen
    print("\nWelcome to the Rhino configuration script")
    print("---")
    print(
        "The Rhino Configuration script allows for you to customise the rhino-update utility and extend its capabilities, such as allowing for the installation of unsupported software."
    )
    print(
        "Please be cautious when using rhino-config, issues can arise from some of the settings so please ensure that you know what you are doing and have read the documentation."
    )
    print("---")
    # Give the user the choice of installing the latest mainline kernel and take user input
    mainline = input("Do you wish to install the latest Linux mainline kernel? [Y/n] ")
    # If user input is yes, go to the mainline install function
    if mainline == "Y" or mainline == "y" or mainline == "":
        mainlineInstall()
    # If user input is no, go to the mainline denied function
    elif mainline == "n" or mainline == "N":
        mainlineDenied()
    print("---")
    snapd = input(
        "Do you wish to remove Snapcraft (snapd) and replace it with Flatpaks? [Y/n]"
    )
    if snapd == "Y" or snapd == "y" or snapd == "":
        snapdPurge()
    elif snapd == "n" or snapd == "N":
        snapdKeep()

    # Print that the program has completed and then exit
    print(
        "\nrhino-config has completed, please run rhino-update to update your system!"
    )
    quit()


# Call the main program
config()
