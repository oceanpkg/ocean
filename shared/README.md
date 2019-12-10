<div align="center">
  <a href="www.oceanpkg.org">
    <img src="https://www.oceanpkg.org/static/images/ocean-logo.svg"
         alt="Ocean logo"
         height="120" width="120">
  </a>
  <br>
  <h1>Ocean Shared Library</h1>
  <a href="https://travis-ci.com/oceanpkg/ocean">
    <img src="https://travis-ci.com/oceanpkg/ocean.svg?branch=master"
         alt="Travis CI badge">
  </a>
  <a href="https://crates.io/crates/oceanpkg-shared">
    <img src="https://img.shields.io/crates/v/oceanpkg-shared.svg"
         alt="Crates.io badge">
  </a>
</div>
<br>

The `oceanpkg-shared` library serves as reusable components for:
- the `oceanpkg` library
- The `ocean` [CLI] client
- Backend web services

## Usage

This library is primarily meant for Ocean's components and not for external use.
Because of this, you won't see parts of this library be publicly re-exported
through the `oceanpkg` library.

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
[crate]: https://crates.io/crates/oceanpkg-shared
[documentation]: https://docs.rs/oceanpkg-shared
[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
