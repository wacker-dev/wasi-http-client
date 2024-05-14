#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use serde::Deserialize;
use std::time::Duration;
use wasi_http_client::Client;

struct Component;

bindings::export!(Component with_types_in bindings);

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Data {
    origin: String,
    url: String,
}

impl Guest for Component {
    fn run() -> Result<(), ()> {
        // get with query
        let resp = Client::new()
            .get("https://httpbin.org/get?a=b")
            .send()
            .unwrap();
        println!(
            "GET https://httpbin.org/get, status code: {}, body:\n{}",
            resp.status(),
            String::from_utf8_lossy(resp.body())
        );

        // get with json response
        let resp = Client::new().get("https://httpbin.org/get").send().unwrap();
        let json_data = resp.json::<Data>().unwrap();
        println!(
            "GET https://httpbin.org/get, status code: {}, body:\n{:?}\n",
            resp.status(),
            json_data,
        );

        // post with json data
        let resp = Client::new()
            .post("https://httpbin.org/post")
            .header("Content-Type", "application/json")
            .unwrap()
            .body("{\"data\": \"hello\"}".as_bytes())
            .connect_timeout(Duration::from_secs(5))
            .send()
            .unwrap();
        println!(
            "POST https://httpbin.org/post, status code: {}, body:\n{}",
            resp.status(),
            String::from_utf8_lossy(resp.body())
        );

        // post with form data
        let resp = Client::new()
            .post("https://httpbin.org/post")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .unwrap()
            .body("a=b&c=".as_bytes())
            .connect_timeout(Duration::from_secs(5))
            .send()
            .unwrap();
        println!(
            "POST https://httpbin.org/post, status code: {}, body:\n{}",
            resp.status(),
            String::from_utf8_lossy(resp.body())
        );

        // post with file form data
        let resp = Client::new()
            .post("https://httpbin.org/post")
            .header("Content-Type", "multipart/form-data; boundary=boundary")
            .unwrap()
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
        println!(
            "POST https://httpbin.org/post, status code: {}, body:\n{}",
            resp.status(),
            String::from_utf8_lossy(resp.body())
        );
        Ok(())
    }
}
