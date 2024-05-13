# Examples

See https://github.com/wacker-dev/wasi-examples/tree/main/http-client for a real-world example.
After compilation, you can use [wasmtime](https://github.com/bytecodealliance/wasmtime) to run it:

```
$ wasmtime -S http target/wasm32-wasi/debug/http_client.wasm

status code: 200
content-type: application/json
content-length: 297
access-control-allow-credentials: true
server: gunicorn/19.9.0
date: Sat, 11 May 2024 09:46:38 GMT
access-control-allow-origin: *
body:
{
  "args": {},
  "data": "hello",
  "files": {},
  "form": {},
  "headers": {
    "Content-Length": "5",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-663f3e7e-6e3f84f87a20aef56c58a344"
  },
  "json": null,
  "origin": "……",
  "url": "https://httpbin.org/post"
}
```

There are specific steps for compilation in the [README](https://github.com/wacker-dev/wasi-examples/blob/main/http-client/README.md),
and the main logic of the sample program is located at [lib.rs](https://github.com/wacker-dev/wasi-examples/blob/main/http-client/src/lib.rs#L14-L19).
