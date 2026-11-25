# EasyELF

EasyELF is a command-line ELF file inspector written in Rust. It displays selected ELF metadata, program headers, dynamic entries, and shared-library dependencies. It can also ask DeepSeek for package-installation suggestions for a selected Linux distribution.

## Build and install

```sh
cargo install --path .
```

This installs the `eelf` command in Cargo’s binary directory (`~/.cargo/bin`).

## Usage

```sh
eelf --path ./program --head --ph --sh --dependencies
```

Options:

- `-p, --path <PATH>` — ELF file to inspect.
- `--head` — show ELF identification and header information.
- `--ph` — show program headers.
- `--sh` — show dynamic entries.
- `-d, --dependencies` — show shared-library dependencies.
- `-a, --ai <DISTRO>` — request package-installation suggestions for a distribution, such as `debian`.
- `-t, --token <TOKEN>` — save a DeepSeek API token for AI suggestions.

The token is saved to `eelf-token.txt` in the current working directory. Run EasyELF from that directory when using `--ai` so it can load the saved token.
