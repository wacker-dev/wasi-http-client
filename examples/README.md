# Examples

See https://github.com/wacker-dev/wasi-examples/tree/main/http-client for a real-world example.
After compilation, you can use [wasmtime](https://github.com/bytecodealliance/wasmtime) to run it:

```
$ wasmtime -S http target/wasm32-wasi/debug/http_client.wasm

GET https://httpbin.org/get, status code: 200, body:
{
  "args": {
    "a": "b"
  },
  "headers": {
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-6642db93-63a3489711d4d5247435ace8"
  },
  "origin": "...",
  "url": "https://httpbin.org/get?a=b"
}

POST https://httpbin.org/post, status code: 200, body:
{
  "args": {},
  "data": "{\"data\": \"hello\"}",
  "files": {},
  "form": {},
  "headers": {
    "Content-Length": "17",
    "Content-Type": "application/json",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-6642db96-4cc2ff4638eab42f4bd8d27f"
  },
  "json": {
    "data": "hello"
  },
  "origin": "...",
  "url": "https://httpbin.org/post"
}

POST https://httpbin.org/post, status code: 200, body:
{
  "args": {},
  "data": "",
  "files": {},
  "form": {
    "a": "b",
    "c": ""
  },
  "headers": {
    "Content-Length": "6",
    "Content-Type": "application/x-www-form-urlencoded",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-6642db98-02c2f6a008c804fe46145f9c"
  },
  "json": null,
  "origin": "...",
  "url": "https://httpbin.org/post"
}

POST https://httpbin.org/post, status code: 200, body:
{
  "args": {},
  "data": "",
  "files": {
    "field2": "hello"
  },
  "form": {
    "field1": "value1"
  },
  "headers": {
    "Content-Length": "156",
    "Content-Type": "multipart/form-data; boundary=boundary",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-6642db99-2f1a72b618ad642e2ceee3a2"
  },
  "json": null,
  "origin": "...",
  "url": "https://httpbin.org/post"
}
```

There are specific steps for compilation in the [README](https://github.com/wacker-dev/wasi-examples/blob/main/http-client/README.md),
and the main logic of the sample program is located at [lib.rs](https://github.com/wacker-dev/wasi-examples/blob/main/http-client/src/lib.rs).
