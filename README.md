# envmn — Environment Manager for `.env`-style Files

`envmn` is a lightweight command-line utility for managing complex `.env` files.
It helps you lint, format, and switch between environment blocks — like `dev` and `prod` — safely and predictably.

> **Note:** `envmn` currently supports **Linux** and **Windows**.

---

## Features

* **Block-based structure** — group related variables into labeled sections
* **Switch between environments** — move a block (e.g., `prod_database`) to the bottom to make it active
* **Tags** — annotate blocks with resource tags like `[db]` or `[smtp]`, so one environment name can cover several resources
* **Lint & format** — check for malformed lines, duplicates, and inconsistent formatting
* **Pipe-friendly** — read from stdin or directly modify files in place
* **Human-readable output** — no noise, just clean `.env` management

---

## Installation

Download the latest binary for your platform from the [**Releases** page](https://github.com/devark28/envmn/releases/latest).

### Linux

```bash
mv envmn_linux_x64 envmn
chmod +x envmn
sudo mv envmn /usr/local/bin/
```

### Windows

1. Download `envmn_windows-gnu_x64.exe` from the [**Releases** page](https://github.com/devark28/envmn/releases/latest).
2. Rename it to `envmn.exe` (optional, for convenience).
3. Add its location to your `PATH` environment variable:
   - Open **System Properties** → **Environment Variables**.
   - Under **System variables**, select `Path` and click **Edit**.
   - Add the folder where you placed `envmn.exe`.
4. Open a new Command Prompt or PowerShell window and verify:

```powershell
envmn version
```

### Verify (Linux)

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
                        (use --tag/-t to narrow the match when several blocks share a name)

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

## Tags

Block headers can carry **tags** in square brackets:

```bash
#@ local [db]
DB_HOST=localhost
##

#@ local [smtp]
MAILGUN_API_KEY=key-xyz123456789
##

#@ remote [db]
DB_HOST=example.com
##
```

The block name is the environment (`local`, `remote`) and the tags describe the resource (`db`, `smtp`).
Tag names follow the same rules as block names: lowercase letters, digits, and underscores, not starting with a digit.

Two blocks may share a name as long as their tags differ. `list` shows tags next to each block name:

```
Blocks (4):
- default
- local [db]
- local [smtp]
- remote [db]
```

### Picking with tags

Blocks that share a tag form a **group** competing for the same variables, and the **last block of a group is the active one**. Picking a tagged block doesn't send it to the bottom of the file — it slides to just **after the last block of its group**, so related blocks stay together and the rest of the file doesn't move:

```bash
envmn pick remote --tag db .env  # 'remote [db]' slides right after the last db block
envmn pick remote .env           # picks ALL blocks named remote, each within its own group
```

A bare `pick <name>` switches the whole environment at once: every block with that name becomes active within its group. Use `--tag` (repeatable) to switch a single resource. Picking a block that is already active is a no-op. Untagged blocks keep the classic behavior and move to the bottom of the file.

Every tagged pick reports what changed on stderr (stdout stays clean for piping), so switches that happen implicitly — e.g. a multi-tag block activating a resource you didn't name — are always visible:

```
$ envmn pick local --tag db .env
db: 'local [db]' now active (was 'remote [db]')
$ envmn pick local --tag db .env
'local' is already active
```

### Recommended layout

`envmn` is opinionated: give each block **one resource tag**, and use the block *name* to bundle resources into an environment:

```bash
#@ remote [db]
DB_HOST=example.com
##

#@ remote [cache]
CACHE_HOST=example.com
##
```

`envmn pick remote` then switches db and cache together, and `envmn pick other --tag cache` later swaps just the cache — any mix of environments is a sequence of picks.

Multiple tags on one block (`#@ remote [db, cache]`) are supported and mean "these resources always switch together as one unit": picking such a block slides it after *all* blocks it shares a tag with, activating everything it defines. Prefer split single-tag blocks unless you really want that all-or-nothing behavior, since a multi-tag block duplicates the variables of every group it belongs to.

### Reserved tags

Tags of the form `__name__` are reserved for special meanings. The only one recognized today is
`__encrypted__`: it marks a block whose **values** are ciphertext. Keys stay plaintext — keys are
rarely the secret, values are — so encrypted blocks still parse, list, format, and lint like any
other block:

```bash
#@ local [smtp, __encrypted__]
MAILGUN_API_KEY=08debe3d42ade91671f783a784fbed31dd3e897abae5439be07f885887217136
MAILGUN_DOMAIN=eeda8bcff5d676460d8ea84bd0e941e4bf5ac466aa21e2512cc1a870e89fb17d
##
```

Encryption and decryption themselves are not implemented yet; the tag currently documents that the
values are not usable as-is.

---

## Other Commands

### Lint

Check for syntax and formatting errors:

```bash
envmn lint .env
```

For tagged files, lint also checks **variable symmetry**: blocks sharing a resource tag compete for the same variables, so they should define the same keys. Single-tag blocks are the ground truth for their tag; a multi-tag block is checked against the union of its tags — extra keys are only flagged when every one of its tags has a single-tag block to learn from. Findings are advisory: they print as warnings on stderr and the exit code stays 0.

```
warning: 'remote [db]' is missing variable 'DB_PORT' defined by its group
warning: 'remote [db, cache]' defines variable 'FOO' that belongs to none of its tags
```

Encrypted blocks are checked too: their keys stay plaintext, only their values are ciphertext.

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

* **Detects labeled blocks** marked with `#@ block_name` (optionally tagged: `#@ block_name [tag1, tag2]`) and closed by `##`
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

