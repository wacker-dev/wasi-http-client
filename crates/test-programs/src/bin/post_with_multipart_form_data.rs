use std::time::Duration;
use wasi_http_client::Client;

fn main() {
    let resp = Client::new()
        .post("https://httpbin.org/post")
        .header("Content-Type", "multipart/form-data; boundary=boundary")
        .body(
            "--boundary
Content-Disposition: form-data; name=field1

value1
--boundary
Content-Disposition: form-data; name=field2; filename=file.txt
Content-Type: text/plain

hello
--boundary--"
                .as_bytes(),
        )
        .connect_timeout(Duration::from_secs(5))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
}
