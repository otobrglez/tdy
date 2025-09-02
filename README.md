# `tdy` - a very opinionated day tracker

`tdy` is a tool for writing and organising daily notes.

## Rules

1. One day, one file.
2. Respect and use `$EDITOR` for writing notes.
3. Use [Markdown](https://www.markdownguide.org/).
5. Use `namespaces` for multiple projects in a person's life.
6. **Simple**, **quick**, and non-invasive. **Then smart.**
7. Cloud-ready and cloud-optional!

## Usage

```bash
tdy open
```

**That's it.** Behind the scenes, `tdy` creates a new [markdown](https://en.wikipedia.org/wiki/Markdown) document in a temporary folder with a simple pre-defined template. Boot up your favourite text editor and wait for you to finish. After the editor is closed, it stores the file in your file system's `$TDY_FILES` (`.tdy` - by default) folder.

`tdy` names files with the following template `<namespace>-<year>-<month>-<date>.md`.

`tdy` respects your `ENV` and reads `EDITOR`, `TDY_FILES`, and `SHELL` respectfully.

```
Usage: tdy open [OPTIONS] --editor <EDITOR>

Options:
  -n, --namespace <NAMESPACE>  [env: NAMESPACE=] [default: tdy]
  -t, --title <TITLE>
  -d, --date <DATE>
      --tdy-files <TDY_FILES>  [env: TDY_FILES=.days/] [default: .days]
      --editor <EDITOR>        [env: EDITOR=nvim]
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

Opens a new document for **today**. The file will have a name similar to `tdy-2025-09-02.md`. If the file does not exist, it will be created beforehand; otherwise, the existing file is opened for editing in your editor. The file is stored in the `./days` folder unless overwritten by `TDY_FILES` or by setting the flag `--tdy-files`.

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

Open or create a new document for **last friday** with namespace set to `work`.

```bash
$ tdy open -n work -d "last friday"
```


## Default template

If the file for the day does not yet exists, it will create a new file with the following markdown template (with the current date-time!):

```markdown
---
date: 2023-06-17
---
# 2023-06-17
```


## Development

This is a Rust project. To compile it you need rust toolchain and then:

```bash
cargo build
cargo build --release
./target/debug tdy open -n hacking
```

## Author

- [Oto Brglez](https://github.com/otobrglez)
