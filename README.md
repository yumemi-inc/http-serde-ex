# [Serde][serde] support for the HTTP crate

Adds ability to serialize and deserialize types from the [HTTP][http] crate.

If you want to serialize `Request` or `Response`, use `into_parts()` and serialize their parts, and then rebuild them using their `Builder`.

[serde]: https://lib.rs/serde
[http]: https://lib.rs/http

## Usage

You must annotate fields with `#[serde(with = "http_serde::<appropriate method>")]`.

```rust
#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "http_serde::method")]
    status: Method,

    #[serde(with = "http_serde::status")]
    status: StatusCode,

    #[serde(with = "http_serde::uri")]
    uri: Uri,

    #[serde(with = "http_serde::header_map")]
    headers: HeaderMap,
}
```

## Requirements

Tested with Rust 1.41.

