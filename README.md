# envmn — Environment Manager for `.env`-style Files

`envmn` is a lightweight command-line utility for managing complex `.env` files.
It helps you lint, format, and switch between environment blocks — like `dev` and `prod` — safely and predictably.

> **Note:** `envmn` currently supports **Linux** only.

---

## Features

* **Block-based structure** — group related variables into labeled sections
* **Switch between environments** — move a block (e.g., `prod_database`) to the bottom to make it active
* **Lint & format** — check for malformed lines, duplicates, and inconsistent formatting
* **Pipe-friendly** — read from stdin or directly modify files in place
* **Human-readable output** — no noise, just clean `.env` management

---

## Installation

Download the latest binary from the [**Releases** page](https://github.com/devark28/envmn/releases/latest).
Then make it executable and move it into your `$PATH`:

```bash
# https://github.com/devark28/envmn/releases/latest and download an envmn binary
# unzip the binary
mv envmn_x64 envmn
chmod +x envmn
sudo mv envmn /usr/local/bin/
```

Verify the installation:

```bash
envmn version
```

---

## Usage

```
envmn — environment manager for .env-style files

Usage:
  envmn <command> [options] [file]

Commands:
  help                  Show this help message
  version               Display the current version
  list                  List all environment blocks in the file
  lint                  Check for syntax and linting errors
  format                Pretty-format the file
  pick <block>          Reorder the file by moving the specified block down

Input modes:
  - If data is piped in, envmn reads from standard input and writes to standard output.
  - If both a pipe and a file are provided, the piped input takes priority.
  - If no file is provided, envmn assumes a `.env` file exists in the current directory (for convenience).
  - When a file path is provided (or .env is assumed), envmn reads from (and edits, if a file was passed) the file directly.
```

---

## Example `.env` File

```bash
# Basic API configuration
API_URL=https://api.example.com
API_KEY=123456789abcdef
DEBUG=true

#@ prod_database
DB_HOST=localhost
DB_PORT=5432
DB_USER=admin
DB_PASSWORD=password123
DB_NAME=mydatabase
##

#@ dev_database
DB_HOST=example.com
DB_PORT=5432
DB_USER=admin
DB_PASSWORD=remote1234
DB_NAME=myappproddatabase
##

#@ email_block
MAILGUN_API_KEY=key-xyz123456789
MAILGUN_DOMAIN=mg.example.com
##
```

Here, each **block** (between `#@` and `##`) defines a related set of environment variables.
In this case, you have separate configurations for **production** and **development** databases.

---

## Switching Environments

Use the `pick` command to move a specific block (e.g. `prod_database`) to the **bottom** of the file.
Because variables are parsed in order, the **last block overrides** earlier ones — effectively switching environments.

```bash
envmn pick prod_database .env.example
```

### Before

```bash
# ... dev block comes last
#@ prod_database
DB_HOST=localhost
...
##
#@ dev_database
DB_HOST=example.com
...
##
```

### After

```bash
# ... prod block moved last (now active)
#@ dev_database
DB_HOST=example.com
...
##
#@ prod_database
DB_HOST=localhost
...
##
```

Now, all the `DB_*` variables from `prod_database` override the ones from `dev_database`.

---

## Other Commands

### Lint

Check for syntax and formatting errors:

```bash
envmn lint .env
```

### Format

Reformat and clean up your `.env` file:

```bash
envmn format .env
```

### List

List all block names in the file:

```bash
envmn list .env
```

### Help

Display the built-in help:

```bash
envmn help
```

---

## How It Works (Technical Overview)

`envmn` parses `.env` files using a small Rust engine that:

* **Detects labeled blocks** marked with `#@ block_name` and closed by `##`
* **Normalizes variable lines** (trims whitespace, fixes quoting issues)
* **Validates** each variable name and key/value format
* **Applies block precedence**: variables from later blocks overwrite earlier definitions

The design allows for **predictable overrides** — switching environments becomes as simple as reordering blocks.

---

## Contributing

Contributions, ideas, and feedback are welcome!
Fork the repo, make your changes, and open a PR.
N.B. this code might be spaghetti, it's because this is my second Rust project

---

## License

MIT License © 2025 [devark28](https://github.com/devark28)

