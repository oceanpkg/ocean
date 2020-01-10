<div align="center">
  <a href="www.oceanpkg.org">
    <img src="https://www.oceanpkg.org/static/images/ocean-logo.svg"
         alt="Ocean logo"
         height="120" width="120">
  </a>
  <br>
  <h1>Ocean Command-Line Interface</h1>
  <a href="https://github.com/oceanpkg/ocean/actions?query=workflow%3ACI">
    <img src="https://github.com/oceanpkg/ocean/workflows/CI/badge.svg"
         alt="Build Status">
  </a>
</div>
<br>

The `ocean` [CLI] client is the main way of using Ocean.

<!--
TODO: Wrap "working directory" in a link to somewhere that explains the term.
-->
**Note:** All shell commands assume that the current working directory is `cli`.
This can be done by running `cd cli` to "change directory" from the root folder.

## Build

To build `ocean` without running it immediately, simply replace `run` with
`build`:

```sh
cargo build
```

This will generate a binary at `../target/debug/ocean`.

To build with optimizations, add the `--release` flag. The binary will then be
made available at `../target/release/ocean`.

Notice that the default build folder is `../target`. To change this, use the
`--target-dir` option.

## Run

This client is written in [Rust] and is built with [`cargo`]. See [rustup.rs]
for installing Rust and `cargo`.

To run _without_ optimizations, run:

```sh
cargo run
```

To build _with_ optimizations, run:

```sh
cargo run --release
```

### Run With Arguments

Both of the above examples will simply output a help message and exit with a
non-0 code.

To pass arguments via `cargo`, place them after a lone `--`:

```sh
cargo run -- install [FLAGS] [OPTIONS] <drop>...
```

Otherwise, arguments can be passed to the compiled binary directly:

```sh
../target/debug/ocean install [FLAGS] [OPTIONS] <drop>...
```

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[Rust]: https://www.rust-lang.org
[`cargo`]: https://doc.rust-lang.org/cargo
[rustup.rs]: https://rustup.rs
[crate]: https://crates.io/crates/oceanpkg
[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
