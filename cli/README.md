<div align="center">
  <a href="www.oceanpkg.org">
    <img src="https://www.oceanpkg.org/static/images/ocean-logo.svg"
         alt="Ocean logo"
         height="120" width="120">
  </a>
  <br>
  <h1>Ocean Command-Line Interface</h1>
  <a href="https://travis-ci.com/oceanpkg/ocean">
    <img src="https://travis-ci.com/oceanpkg/ocean.svg?branch=master"
         alt="Travis CI badge">
  </a>
</div>
<br>

The `ocean` [CLI] client is the main way of using Ocean.

<!--
TODO: Wrap "working directory" in a link to somewhere that explains the term.
-->
**Note:** All shell commands assume that the current working directory is `cli`.
This can be done by running `cd cli` to "change directory" from the root folder.

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

Both of these will simply output a help message and exit with a non-0 code. To
pass arguments via `cargo`, run:

```sh
ocean run -- install [FLAGS] [OPTIONS] <drop>...
```

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

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[Rust]: https://www.rust-lang.org
[`cargo`]: https://doc.rust-lang.org/cargo
[rustup.rs]: https://rustup.rs
[crate]: https://crates.io/crates/oceanpkg
[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html