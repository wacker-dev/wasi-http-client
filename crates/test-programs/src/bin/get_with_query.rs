use wasi_http_client::Client;

fn main() {
    let resp = Client::new()
        .get("https://httpbin.org/get?a=b")
        .headers([("Content-Type", "application/json"), ("Accept", "*/*")])
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
}
