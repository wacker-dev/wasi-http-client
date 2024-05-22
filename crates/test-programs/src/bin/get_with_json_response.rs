use serde::Deserialize;
use wasi_http_client::Client;

#[derive(Deserialize)]
struct Data {
    url: String,
}

fn main() {
    let resp = Client::new().get("https://httpbin.org/get").send().unwrap();
    let status = resp.status();
    assert_eq!(status, 200);

    let json_data = resp.json::<Data>().unwrap();
    assert_eq!(json_data.url, "https://httpbin.org/get")
}
