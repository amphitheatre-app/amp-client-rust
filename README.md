# Amphitheatre Rust Client

Represents the Rust client for the Amphitheatre API.

The client is your entrypoint to the Amphitheatre API. Using it you will be
able to call all the enpoints of the Amphitheatre API and their respective functions.

## Examples

```rust
use client::client::Client;

let client = Client::new(
    String::from("https://cloud.amphitheatre.app"),
    String::from("AUTH_TOKEN"),
);
let response = client.accounts().me().unwrap();
let account = response.data.unwrap();
```

## License

Licensed under the [Apache License 2.0](https://github.com/amphitheatre-app/amp-client-rust/blob/master/LICENSE)
