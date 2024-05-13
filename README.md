# wasi-http-client

HTTP client library for [WASI Preview 2](https://github.com/WebAssembly/WASI/tree/main/preview2),
making it easier to send http(s) requests in WASI components.

```rust
let resp = Client::new()
    .post("https://httpbin.org/post")
    .body("hello".as_bytes())
    .connect_timeout(Duration::from_secs(5))
    .send()
    .unwrap();

println!("status code: {}", resp.status());
```
