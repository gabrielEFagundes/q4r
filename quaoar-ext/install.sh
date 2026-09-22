# this installation is for dev only, the final extension will be available
# on VSCode's extensions store, or equivalent (VSCodium, Cursor, etc)

# currently only available for VSCode

NO_FORMAT="\033[0m"
F_BOLD="\033[1m"
C_YELLOW3="\033[38;5;184m"
C_GREEN3="\033[38;5;40m"

TARGET="$HOME/.vscode/extensions"
EXTENSION_NAME="gabrielEFagundes.$(dirname -- "$0")-0.1.0"
EXT="$(pwd)/$(dirname -- "$0")"

echo -e "${F_BOLD}Installing quaoar's VSCode extension...${NO_FORMAT}"
echo -e "
${C_YELLOW3}WARNING:${NO_FORMAT} 
This extension is IN DEVELOPMENT. Open issues at https://github.com/gabrielEFagundes/q4r 
to report a problem or an enhancement.
"

cp -r $EXT $TARGET
mv "$TARGET/$(dirname -- "$0")" "$TARGET/$EXTENSION_NAME"

echo -e "${F_BOLD}${C_GREEN3}Installed extension to VSCode! Restart your editor if you see no changes${NO_FORMAT}"