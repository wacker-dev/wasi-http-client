use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use wasi_http_client::Client;

#[derive(Deserialize)]
struct Data {
    json: HashMap<String, String>,
}

fn main() {
    let resp = Client::new()
        .post("https://httpbin.org/post")
        .json(&HashMap::from([("data", "hello")]))
        .connect_timeout(Duration::from_secs(5))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);

    let data = resp.json::<Data>().unwrap();
    assert_eq!(data.json.get("data").unwrap(), "hello");
}
