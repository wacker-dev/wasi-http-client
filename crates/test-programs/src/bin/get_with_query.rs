use serde::Deserialize;
use std::collections::HashMap;
use wasi_http_client::Client;

#[derive(Deserialize)]
struct Data {
    args: HashMap<String, String>,
}

fn main() {
    let resp = Client::new()
        .get("https://httpbin.org/get?a=b")
        .headers([("Content-Type", "application/json"), ("Accept", "*/*")])
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);

    let data = resp.json::<Data>().unwrap();
    assert_eq!(data.args.get("a").unwrap(), "b");
}
