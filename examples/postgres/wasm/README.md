# diesel-async on WebAssembly

Using `diesel-async` with the `postgres` feature on the `wasm32-unknown-unknown`
target.

`AsyncPgConnection::establish` is not available on this target because it has no
TCP socket. The connection is instead built from a `tokio_postgres` client and
connection supplied by the host environment (for example a socket handed out by
an edge runtime such as Cloudflare Workers) via
`AsyncPgConnection::try_from_client_and_connection`.

The host-provided stream must be `Send`, which edge and serverless runtimes
satisfy but browsers generally do not.

## Building

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```
