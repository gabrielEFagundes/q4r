# this installation is for dev only, the final extension will be available
# on VSCode's extensions store, or equivalent (VSCodium, Cursor, etc)

# currently only available for VSCode
NO_FORMAT="\033[0m";
F_BOLD="\033[1m";
C_YELLOW3="\033[38;5;184m";
C_GREEN3="\033[38;5;40m";

TARGET="$HOME/.vscode/extensions";
EXTENSION_NAME="gabrielEFagundes.quaoar_extension_pack-0.1.0";
EXT="$(pwd)/quaoar-ext";

# no heredoc since I can't find a way to keep the formatting
echo -e "${F_BOLD}
This script assumes that you're running on a BASH shell environment.

If you're not, then consider using Git Bash on Windows, '/bin/bash' on Linux or 'bash' on macOS
${NO_FORMAT}
"

echo -e "${F_BOLD}Installing quaoar's VSCode extension...${NO_FORMAT}";
echo -e "
${C_YELLOW3}WARNING:${NO_FORMAT} 
This extension is IN DEVELOPMENT. Open issues at https://github.com/gabrielEFagundes/q4r 
to report a problem or an enhancement.
"

sleep 1

mkdir tmp;
cp -r $EXT tmp;

if [ -e "$TARGET/$EXTENSION_NAME" ]; then
    echo "Directory for quaoar extension already exists, reinstalling...";
    rm -rf "$TARGET/$EXTENSION_NAME";
fi

mv "tmp/quaoar-ext" "$TARGET/$EXTENSION_NAME";

rm -rf tmp;

echo -e "${F_BOLD}${C_GREEN3}Installed extension to VSCode! Restart your editor if you see no changes${NO_FORMAT}";