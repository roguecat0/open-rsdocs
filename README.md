# RustDoc Search CLI

A simple command-line tool for quickly opening documentation pages for Rust libraries using the [`clap`](https://docs.rs/clap/latest/clap/) crate.

This tool opens the appropriate Rust documentation URL in your default web browser, either for the standard library or any crate on [docs.rs](https://docs.rs/).

## Features

- Search the [Rust Standard Library](https://doc.rust-lang.org/std/)
- Search third-party crates on [docs.rs](https://docs.rs/)
- Quickly open the crate’s main documentation page

## Usage

```bash
rsdocs <search> [lib]
```

### Arguments

- `<search>` (required): The term you want to search for.  
  Use `.` to open the main page of the crate without searching.
- `[lib]` (optional): The library or crate name to search in.  
  Defaults to `std`.

### Examples

Search for `String` in the standard library:

```bash
rsdocs String
```

Search for `Value` in the `serde_json` crate:

```bash
rsdocs Value serde_json
```

Open the main documentation page for the `tokio` crate:

```bash
rsdocs . tokio
```
## Dependencies

- [`clap`](https://docs.rs/clap/latest/clap/)
- [`anyhow`](https://docs.rs/anyhow/latest/anyhow/)
- [`webbrowser`](https://docs.rs/webbrowser/latest/webbrowser/)

## License

MIT
