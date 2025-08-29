# nshell - NT Style Shell

A modern implementation of the classic Windows NT command shell with enhanced functionality.

## Features

- Classic NT command prompt look and feel
- Cross-platform compatibility (Windows, Linux, macOS)
- Python backend for advanced functionality
- VBScript integration for Windows-specific features
- Customizable prompt and color schemes
- Command history and tab completion
- Built-in system information tools

## Requirements

- Windows, Linux, or macOS
- Python 3.6+
- Bash (for Linux/macOS) or PowerShell (for Windows)
- VBScript (Windows only, for advanced features)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/nshell.git
   cd nshell
   ```

2. Install Python dependencies:
   ```
   pip install -r requirements.txt
   ```

3. Make the shell script executable (Linux/macOS):
   ```
   chmod +x bin/nshell.sh
   ```

## Usage

### Starting nshell

- **Windows**:
  ```
  .\bin\nshell.sh
  ```
  
- **Linux/macOS**:
  ```
  ./bin/nshell.sh
  ```

### Basic Commands

- `help` - Show available commands
- `ver` - Show version information
- `cls` - Clear the screen
- `exit` - Exit nshell
- `dir` - List directory contents
- `cd` - Change directory
- `type` - Display file contents
- `system` - Show system information
- `processes` - List running processes
- `network` - Show network information
- `disk` - Show disk usage

## Configuration

Edit `config/nshell.conf` to customize nshell behavior. You can modify:
- Prompt style
- Color scheme
- History settings
- Feature toggles
- Custom aliases

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
