<div align="center">
  <a href="www.oceanpkg.org">
    <img src="https://www.oceanpkg.org/static/images/ocean-logo.svg"
         alt="Ocean logo"
         height="120" width="120">
  </a>
  <br>
  <h1>Ocean Library</h1>
  <a href="https://travis-ci.com/oceanpkg/ocean">
    <img src="https://travis-ci.com/oceanpkg/ocean.svg?branch=master"
         alt="Travis CI badge">
  </a>
  <a href="https://crates.io/crates/oceanpkg">
    <img src="https://img.shields.io/crates/v/oceanpkg.svg"
         alt="Crates.io badge">
  </a>
</div>
<br>

The `oceanpkg` library serves as core reusable components for:
- The `ocean` [CLI] client
- Backend web services

<!--
TODO: Wrap "working directory" in a link to somewhere that explains the term.
-->
**Note:** All shell commands assume that the current working directory is `lib`.
This can be done by running `cd lib` to "change directory" from the root folder.

## Install

This library is written in [Rust] and is meant to be used within a [`cargo`]
project. See [rustup.rs] for installing Rust and `cargo`.

It is made available [on crates.io][crate] and can be used by adding the
following to your project's [`Cargo.toml`]:

```toml
[dependencies]
oceanpkg = "0.0.9"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
extern crate oceanpkg;
```

## Usage

See [documentation].

## Testing

Various test cases are covered throughout this library. They can all be found by
searching for `mod tests` within the `lib` folder.

To perform these tests, simply run:

```sh
cargo test
```

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[Rust]: https://www.rust-lang.org
[`cargo`]: https://doc.rust-lang.org/cargo
[rustup.rs]: https://rustup.rs
[crate]: https://crates.io/crates/oceanpkg
[documentation]: https://docs.rs/oceanpkg
[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
