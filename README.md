# `imurl` Immutable URL Wrapper in Rust

[![Crates.io](https://img.shields.io/crates/v/imurl.svg)](https://crates.io/crates/imurl)
[![Documentation](https://docs.rs/imurl/badge.svg)](https://docs.rs/imurl)

`imurl` is a Rust library that provides an immutable wrapper around the `url::Url` type. It provides a simple API for manipulating URLs, making it easier to work with URLs in Rust applications.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
imurl = "0.0.1"
```

## Usage

To create an `ImUrl` instance, use the `parse` method:

```rust
use imurl::ImUrl;

let url = ImUrl::parse("https://example.com/path?query=value#fragment")?;
```

You can then use the various methods provided by the `ImUrl` struct to manipulate the URL:

```rust
let updated_url = url.with_path("/new/path")?;
let updated_url = url.with_query("key=value")?;
let
