# `backup`

Program for backing up data.

## Table of Contents

- [Installation](#installation)
  - [Step 1 - Install binary](#step-1---install-binary)
  - [Step 2 - Initialize program](#step-2---initialize-program)
  - [Step 3 - Edit configurations](#step-3---edit-configurations)
- [Usage](#usage)
  - [Run backups](#run-backups)
  - [Compare `localhost` with backup destinations](#compare-localhost-with-backup-destinations)
- [License](#license)

## Installation

### Step 1 - Install binary
Install the binary using Cargo:
```sh
cargo install --path . --root ~/.local
```
This installs the `backup` binary to `~/.local/bin`, which should be on your
`PATH`.

### Step 2 - Initialize program
Run the `init` command to generate all necessary program files.
```sh
backup init
```

### Step 3 - Edit configurations
The `init` command will generate a TOML configuration file (among other files).
Edit this file:
```sh
vim ~/.backup/config.toml
```

## Usage

### Run backups
Run:
```sh
backup
```
And select where the data should be backed up to.

### Compare `localhost` with backup destinations
Run:
```sh
backup diff
```
Which will compare the sizes of the source directory to the sizes of the
destination directories.

## License
MIT License — see [`LICENSE`](LICENSE) for details.
