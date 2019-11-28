<!--
Make sure to keep relevant header changes in sync with `lib/README.md` and
`cli/README.md`.
-->
<div align="center">
  <a href="www.oceanpkg.org">
    <img src="https://www.oceanpkg.org/static/images/ocean-logo.svg"
         alt="Ocean logo"
         height="120" width="120">
  </a>
  <br>
  <h1>Ocean</h1>
  <a href="https://travis-ci.com/oceanpkg/ocean">
    <img src="https://travis-ci.com/oceanpkg/ocean.svg?branch=master"
         alt="Travis CI badge">
  </a>
  <img src="https://tokei.rs/b1/github/oceanpkg/ocean" alt="Lines of code">
</div>
<br>

The package manager from the future, coming to an operating system near you!

## Compatibility

| Platform | Status |
| :------- | :----- |
| macOS    | Actively developed |
| Linux    | Actively developed |
| Windows  | Future support planned |

## Command-Line Interface

The `ocean` [CLI] client is the main way of using Ocean.

See [`cli/README.md`] for info.

## Library

![[Crates.io badge](https://crates.io/crates/oceanpkg)](https://img.shields.io/crates/v/oceanpkg.svg)

The `oceanpkg` library serves as core reusable components for:
- The `ocean` [CLI] client
- Backend web services

See [`lib/README.md`] or [docs.rs/oceanpkg] for info.

## License

Ocean is licensed under Apache 2.0. See [`LICENSE.txt`] for full text.

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[`LICENSE.txt`]: https://github.com/oceanpkg/ocean/blob/master/LICENSE.txt
[`cli/README.md`]: https://github.com/oceanpkg/ocean/blob/master/cli/README.md
[`lib/README.md`]: https://github.com/oceanpkg/ocean/blob/master/lib/README.md
[docs.rs/oceanpkg]: https://docs.rs/oceanpkg
