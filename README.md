# Promptify

A command-line tool that generates structured context files for Large Language Models (LLMs) from your project's source code.

## Features

- 📂 Generates a comprehensive directory tree structure
- 📝 Includes file contents in Markdown code blocks
- 🔍 Respects .gitignore rules
- ⚡ Supports file filtering with glob patterns
- 🔒 Built-in file size limits for safety
- 🎨 Customizable preamble and postamble text
- 🚫 Smart binary file detection 

## Building and Installation

### Prerequisites

- Rust and Cargo (1.70.0 or newer recommended)
- Git (for cloning the repository)

### Building from Source

1. Clone this repository:
```bash
git clone [repository-url]
cd promptify
```

2. Build the project:
```bash
cargo build --release
```

The compiled binary will be available at `target/release/promptify`

3. (Optional) Install system-wide:
```bash
# On Linux/macOS
sudo cp target/release/promptify /usr/local/bin/
```

## Usage

Basic usage:

```bash
promptify /path/to/your/project
```

This will create a `prompt.md` file in your current directory containing the project overview.

### Command-line Options

```
Options:
  -o, --output <FILE>           Output file path [default: prompt.md]
      --preamble <TEXT>         Custom preamble text to add at the start
      --preamble-file <FILE>    Path to a file containing the preamble
      --postamble <TEXT>        Custom postamble text to add at the end
      --postamble-file <FILE>   Path to a file containing the postamble
      --no-gitignore           Do not respect VCS ignore files
      --include-binaries       Include binary files (metadata only)
      --include <GLOB>         Glob patterns for files to always include
      --exclude <GLOB>         Glob patterns for files to explicitly exclude
      --max-file-size <SIZE>   Maximum file size (e.g., "1M", "512k")
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

Generate context from a specific directory:
```bash
promptify ~/myproject
```

Output to a custom file:
```bash
promptify . -o context.md
```

Include only Python files:
```bash
promptify . --include "**/*.py"
```

Exclude test files:
```bash
promptify . --exclude "**/*test*"
```

Set a file size limit:
```bash
promptify . --max-file-size 500k
```

Output directly to stdout:
```bash
promptify . -o -
```

## Output Format

The generated file includes:

1. A preamble explaining the content
2. A directory tree showing the project structure
3. File contents in Markdown code blocks (using triple backticks)
4. A postamble with instructions for the LLM

### Binary File Handling

By default, Promptify skips binary files to keep the output clean and prevent issues with non-text content. However, you can:

1. Use `--include-binaries` to include binary files in the output:
   - Binary files will be noted with `[Binary File: Content not shown]`
   - File paths and sizes are still included in the directory structure
   - Useful for projects with images, executables, or other binary assets

2. Control which binary files to include/exclude:
   ```bash
   # Include specific binary file types
   promptify . --include-binaries --include "**/*.png"
   
   # Include binaries but exclude certain types
   promptify . --include-binaries --exclude "**/*.exe"
   ```

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

## Copyright

Copyright (C) 2025 Souheab