# Imports
import os

# Change configuration to enable the mainline kernel
def mainlineInstall():
    os.system("touch ~/.rhino/config/mainline")
    print("Configuration updated!")

# Do not enable the mainline kernel
def mainlineDenied():
    print("No changes were made to the Rhino Configuration")

def config():
    # Splash screen
    print("\nWelcome to the Rhino configuration script")
    print("---")
    print("The Rhino Configuration script allows for you to customise the rhino-update utility and extend its capabilities, such as allowing for the installation of unsupported software.")
    print("Please be cautious when using rhino-config, issues can arise from some of the settings so please ensure that you know what you are doing and have read the documentation.")
    print("---")
    # Give the user the choice of installing the latest mainline kernel and take user input
    mainline = input("Do you wish to install the latest Linux mainline kernel? [Y/n] ")
    # If user input is yes, go to the mainline install function
    if mainline == "Y" or mainline == "y" or mainline == "":
        mainlineInstall()
    # If user input is no, go to the mainline denied function
    elif mainline == "n" or mainline == "N":
        mainlineDenied()
    
    # Print that the program has completed and then exit
    print("\nrhino-config has completed, please run rhino-update to update your system!")
    quit()

# Call the script
config()