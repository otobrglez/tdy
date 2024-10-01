# `tdy` - a very opinionated day tracker

`tdy` is a tool for writing and organizing daily notes.

## Rules

1. One day, one file.
2. I want to use my `$EDITOR` for writing notes.
3. I want to write everything as [markdown](https://www.markdownguide.org/).
5. There should be `namespaces` for multiple projects in a person's life.

## Usage

```bash
tdy open
```

**That's it.** Behind the scenes `tdy` creates a new markdown document in a temporary folder with a simple pre-defined template. Boots up your favourite text editor and waits for it to finish. After the editor is closed, it stores the file in your file system's `$TDY_FILES` (`.tdy` - by default) folder.

`tdy` names files with the following template `<namespace>-<year>-<month>-<date>.md`.

`tdy` respects your `ENV` and reads `EDITOR`, `TDY_FILES`, and `SHELL` respectfully.

```
Usage: tdy open [OPTIONS] --editor <EDITOR> --shell <SHELL>

Options:
  -n, --namespace <NAMESPACE>  [env: NAMESPACE=] [default: tdy]
  -t <TITLE>                   [default: ]
  -d, --date <DATE>
      --tdy-files <TDY_FILES>  [env: TDY_FILES=] [default: .days]
      --editor <EDITOR>        [env: EDITOR=vim]
      --shell <SHELL>          [env: SHELL=/bin/zsh]
  -h, --help                   Print help
  -V, --version                Print version
```

## Mission

I use `tdy` for all my daily notes. I want it to be an easy, fast and non-invasive process. **Then smart.**


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
