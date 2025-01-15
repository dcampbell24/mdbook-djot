# mdBook Djot

[![github]][github-link]&ensp;[![crates-io]][crates-io-link]&ensp;[![docs-rs]][docs-rs-link]

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[github-link]: https://github.com/dcampbell24/mdbook-djot
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[crates-io-link]: https://crates.io/crates/mdbook-djot
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[docs-rs-link]: https://docs.rs/mdbook-djot

Djot plugin for mdBook.

## Install

```sh
cargo install mdbook
cargo install mdbook-djot
```

## Example

Check out the [test_book] for an example of how this works.

[test_book]: https://github.com/dcampbell24/mdbook-djot/tree/main/test_book

You just need to add the line `[preprocessor.djot]` to your `book.toml` to use
the plugin. Then all the files that end in `.dj` will be treated as Djot files.
The `SUMMARY` and `404` files have to be markdown.
