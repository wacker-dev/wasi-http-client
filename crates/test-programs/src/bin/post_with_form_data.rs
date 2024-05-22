use std::time::Duration;
use wasi_http_client::Client;

fn main() {
    let resp = Client::new()
        .post("https://httpbin.org/post")
        .form(&[("a", "b"), ("c", "")])
        .connect_timeout(Duration::from_secs(5))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
}
