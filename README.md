# `backup`

Program for backing up data. Ported from Python. The Python program felt clunky
and slow. Additionally, virtual environments were a pain to work with.

## Table of Contents

- [Synopsis](#synopsis)
- [Installation](#installation)
  - [Step 1 - Install binary](#step-1---install-binary)
  - [Step 2 - Initialize program](#step-2---initialize-program)
  - [Step 3 - Edit configurations](#step-3---edit-configurations)
- [Usage](#usage)
  - [Run backups](#run-backups)
  - [Compare `localhost` with backup destinations](#compare-localhost-with-backup-destinations)
- [License](#license)

## Synopsis

This backup system backs data up to two destinations:
<table>
  <thead>
    <tr>
      <th>Target</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Hot storage</td>
      <td>Machine that is hard wired into the network. Data is frequently backed up to this machine to closely mirror the source</td>
    </tr>
    <tr>
      <td>Cold storage</td>
      <td>Machine that is NOT hard wired into the network. Data is infrequently backed up to this machine</td>
    </tr>
  </tbody>
</table>

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
The program will prompt whether to back data up to hot storage or cold storage.
Additionally, the program will prompt whether to perform a dry run which can be
useful for testing an installation.

### Compare `localhost` with backup destinations
Run:
```sh
backup diff
```
Which will compare the sizes of the source directory to the sizes of the
destination directories.

## License
MIT License — see [`LICENSE`](LICENSE) for details.
