[![Continuous Integration](https://github.com/nicklaswallgren/bankid-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/nicklaswallgren/bankd-rs/actions)
[![License](https://img.shields.io/github/license/nicklaswallgren/bankid-rs)](https://github.com/nicklaswallgren/bankid-rs/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/bankd-rs.svg)](https://crates.io/crates/bankid-rs)
[![Docs](https://docs.rs/bankid-rs/badge.svg)](https://docs.rs/crate/bankid-rs/)

# BankID-rs

BankID-rs is a BankID SDK. It includes support for all the v5.1 features.

To learn how to use BankID-rs, please refer to the [documentation](https://docs.rs/crate/bankid-rs/). There are some [examples that may be useful](./examples) as well.

## Changelog

Please see the [changelog](./CHANGELOG.md) for a release history and indications on how to upgrade from one version to another.

## Contributing

If you find any problems or have suggestions about this crate, please submit an issue. Moreover, any pull request, code review and feedback are welcome.

### Code Guide

We use GitHub Actions to make sure the codebase is consistent (`cargo +nightly fmt`) and continuously tested (`cargo test`). We try to keep comments at a maximum of 120 characters of length and code at 120.

## Building

BankID-rs uses [`maybe_async`](https://docs.rs/maybe-async/0.2.0/maybe_async/) to switch between async and blocking clients, which is triggered inside `Cargo.toml`. So that must be taken into account when building `bankid-rs`. Read the Configuration section in the docs for more information about how to build, and more.

```sh
$ cargo build
```

`bankid-rs` is also available as the `blocking` interface

```sh
$ cargo build --features async
```

## License

[MIT](./LICENSE)
