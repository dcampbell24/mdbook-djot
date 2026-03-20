<!-- >
SPDX-FileCopyrightText: 2025 David Lawrence Campbell
SPDX-License-Identifier: MPL-2.0
<-->

# mdBook Djot

[![github]][github-link]&ensp;[![crates-io]][crates-io-link]&ensp;[![docs-rs]][docs-rs-link]&ensp;[![REUSE status]][reuse]&ensp;

[github]: https://img.shields.io/badge/github-8da0cb
[github-link]: https://github.com/dcampbell24/mdbook-djot
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62
[crates-io-link]: https://crates.io/crates/mdbook-djot
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5
[docs-rs-link]: https://docs.rs/mdbook-djot
[REUSE status]: https://api.reuse.software/badge/github.com/dcampbell24/mdbook-djot
[reuse]: https://api.reuse.software/info/github.com/dcampbell24/mdbook-djot

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
