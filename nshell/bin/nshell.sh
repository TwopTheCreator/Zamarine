#!/bin/bash

# nshell - NT Style Shell
# Initialize environment
NSHELL_HOME="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="$NSHELL_HOME/config/nshell.conf"
THEME_FILE="$NSHELL_HOME/themes/default.theme"

# Source config if exists
[ -f "$CONFIG_FILE" ] && source "$CONFIG_FILE"

# Default settings
: ${PROMPT:="C:\\>"}
: ${THEME:="default"}
: ${HISTFILE:="$NSHELL_HOME/.nshell_history"}
: ${HISTFILESIZE:=1000}

# Load theme
load_theme() {
    if [ -f "$THEME_FILE" ]; then
        source "$THEME_FILE"
    fi
}

# Initialize shell
initialize() {
    # Create history file if it doesn't exist
    touch "$HISTFILE"
    # Load theme
    load_theme
    # Display welcome message
    clear
    echo "Microsoft(R) Windows NT
(C) Copyright Microsoft Corp 1990-1999. All rights reserved."
    echo "NSHELL v0.1 - NT Style Shell"
    echo "Type 'help' for available commands"
    echo
}

# Command processing
process_command() {
    local cmd="$1"
    local args=("${@:2}")
    
    case "$cmd" in
        help)
            echo "Available commands:"
            echo "  help      - Show this help"
            echo "  ver       - Show version information"
            echo "  cls       - Clear screen"
            echo "  exit      - Exit nshell"
            echo "  dir       - List directory contents"
            echo "  cd        - Change directory"
            echo "  type      - Display file contents"
            ;;
        ver)
            echo "Microsoft Windows NT [Version 4.0]"
            echo "(C) 1985-1999 Microsoft Corp."
            ;;
        cls)
            clear
            ;;
        exit)
            echo "Exiting nshell..."
            exit 0
            ;;
        dir)
            ls -la --color=auto "${args[@]}"
            ;;
        cd)
            cd "${args[0]:-$HOME}" || echo "The system cannot find the path specified."
            ;;
        type)
            if [ -z "${args[0]}" ]; then
                echo "The syntax of the command is incorrect."
            else
                cat "${args[0]}" 2>/dev/null || echo "File not found"
            fi
            ;;
        *)
            if [ -n "$cmd" ]; then
                if command -v "$cmd" >/dev/null 2>&1; then
                    "$cmd" "${args[@]}"
                else
                    echo "'$cmd' is not recognized as an internal or external command,"
                    echo "operable program or batch file."
                fi
            fi
            ;;
    esac
}

# Main shell loop
main() {
    initialize
    
    while true; do
        # Get current directory in Windows style
        local current_dir="${PWD//\/\\}"
        current_dir="${current_dir//\/\\}"  # Convert forward slashes to backslashes
        current_dir="${current_dir/C:/C:}"  # Ensure drive letter is uppercase
        
        # Read command with readline
        IFS= read -e -p "$current_dir$PROMPT " cmd args
        
        # Add to history
        history -s "$cmd $args"
        
        # Process command
        process_command $cmd $args
    done
}

# Start the shell
main "$@"
