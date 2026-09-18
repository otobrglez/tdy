# `tdy` - a very opinionated day tracker

[![Build](https://github.com/otobrglez/tdy/actions/workflows/build.yml/badge.svg)](https://github.com/otobrglez/tdy/actions/workflows/build.yml)
[![Release](https://github.com/otobrglez/tdy/actions/workflows/release.yml/badge.svg)](https://github.com/otobrglez/tdy/actions/workflows/release.yml)

`tdy` is a tool for writing and organising daily notes.

## Rules and principles

1. One day, one file.
2. Use `namespaces` for multiple projects in a person's life.
3. Bring-your-own editor. Respect and use `$EDITOR`.
4. Quick, and non-invasive. **Then smart.**
5. Use [Markdown](https://www.markdownguide.org/).
6. Cloud-ready and cloud-optional!

## Installation

Pre-built binaries for macOS (Apple Silicon and Intel) and Linux (x86_64 and aarch64) are published on the
[releases page](https://github.com/otobrglez/tdy/releases). The installer script picks the right one and puts `tdy`
into `~/.cargo/bin`:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/otobrglez/tdy/releases/latest/download/tdy-installer.sh | sh
```

Or build from source with `cargo install --git https://github.com/otobrglez/tdy`.

## Usage

```bash
tdy open
```

**That's it.** Behind the scenes, `tdy` creates a new [Markdown](https://en.wikipedia.org/wiki/Markdown) document in a
temporary folder with a simple pre-defined template. Boot up your favourite text editor and wait for you to finish.
After the editor is closed, it stores the file in your file system's `$TDY_FILES` (`.days` - by default) folder. If the
editor exits with an error, nothing is saved.

`tdy` names files with the following template `<namespace>-<YYYY>-<MM>-<DD>.md`. Dates are calendar dates in your
local time zone.

`tdy` respects your `ENV` and reads `EDITOR`, `TDY_FILES` and `NAMESPACE`.

```
Usage: tdy <COMMAND>

Commands:
  open  Open the document for a day in your editor, creating it if needed
  path  Print the path of the document for a day, if it exists
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
Usage: tdy open [OPTIONS] --editor <EDITOR>

Options:
  -n, --namespace <NAMESPACE>  Namespace that groups documents, for example `work` [env: NAMESPACE=] [default: tdy]
  -d, --date <DATE>            Day of the document: `2025-12-31`, `today`, `yesterday`, `tomorrow`, `last friday` or `next monday` [default: today]
      --tdy-files <TDY_FILES>  Directory where documents are stored [env: TDY_FILES=] [default: .days]
  -t, --title <TITLE>          Heading of a newly created document [default: the date]
      --editor <EDITOR>        Editor command used to open the document [env: EDITOR=]
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

Opens a new document for **today**. The file will have a name similar to `tdy-2025-09-02.md`. If the file does not
exist, it will be created beforehand; otherwise, the existing file is opened for editing in your editor. The file is
stored in the `.days` folder unless overwritten by `TDY_FILES` or by setting the flag `--tdy-files`.

```bash
$ tdy open
```

Open or create a new document for **today** with the `namespace` set to `work`.

```bash
$ tdy open -n work
```

Open or create a new document for **yesterday** with namespace set to `work`.

```bash
$ tdy open -n work -d yesterday
```

Open or create a new document for **last friday** with namespace set to `work` and title set to `Friday meeting report`.

```bash
$ tdy open -n work -d "last friday" -t "Friday meeting report"
```

The same works for next Monday.

```bash
$ tdy open -n work -d "next monday" -t "Monday planning."
```

### Finding a document

`tdy path` prints the location of an existing document and prints nothing if there is none. It takes the same
`--namespace`, `--date` and `--tdy-files` options as `tdy open`, which makes it handy in scripts.

```bash
$ tdy path -n work -d yesterday
.days/work-2025-09-01.md

$ cat "$(tdy path -n work -d yesterday)"
```

## Default template

If the file for the day does not yet exist, it will create a new file with the following Markdown template (with the
current date, or the date you asked for):

```markdown
---
date: 2023-06-17
---
# 2023-06-17
```

## Export to PDF

`convert.sh` turns a note into a PDF with [pandoc](https://pandoc.org/) and XeLaTeX. Both, together with the fonts it
uses, are provided by the `devenv` shell.

```bash
./convert.sh .days/work-2025-09-01.md work-2025-09-01.pdf
```

## Development

This is a Rust project. To compile it you need a Rust toolchain (or run `devenv shell`) and then:

```bash
cargo build
cargo build --release
./target/debug/tdy open -n hacking
```

CI runs the same checks you can run locally:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

### Releasing

Releases are built by [dist](https://opensource.axo.dev/cargo-dist/) (`cargo-dist`), which the `devenv` shell
provides. Bump `version` in `Cargo.toml`, commit, and push a matching tag. The `Release` workflow then builds the
macOS and Linux binaries and the installer, and publishes them as a GitHub release.

```bash
git tag v0.0.9
git push origin v0.0.9
```

The release configuration lives in `dist-workspace.toml`. After changing it, run `dist generate` to refresh
`.github/workflows/release.yml` and `dist plan` to check the result.

## Author

- [Oto Brglez](https://github.com/otobrglez)
