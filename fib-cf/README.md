# Fibonacci Rust Cloudflare worker

## Requirements

 - yarn
 - cargo
 - rust

## Running locally

```bash
yarn dev
```

## Deploying

Wrangler is used to develop, deploy, and configure your Worker via CLI.

Further documentation for Wrangler can be found [here](https://developers.cloudflare.com/workers/tooling/wrangler).

Read the latest `worker` crate documentation here: https://docs.rs/worker



[![Deploy with Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/cloudflare/workers-sdk/tree/main/templates/experimental/worker-rust)

A template for kick starting a Cloudflare worker project using [`workers-rs`](https://github.com/cloudflare/workers-rs).

This template is designed for compiling Rust to WebAssembly and publishing the resulting worker to Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Advanced Example

As this template comprises only the essential setup, we recommend considering our advanced example to leverage its additional functionalities. The advanced example showcases the creation of multiple routes, logging of requests, retrieval of field data from a form, and other features that may prove useful to your project.  
The following example has been taken from: [workers-rs](https://github.com/cloudflare/workers-rs). You can learn more about how to use workers with rust by going there.

```rust
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    if !matches!(req.method(), Method::Post) {
        return Response::error("Method Not Allowed", 405);
    }

    if let Some(file) = req.form_data().await?.get("file") {
        return match file {
            FormEntry::File(buf) => {
                Response::ok(&format!("size = {}", buf.bytes().await?.len()))
            }
            _ => Response::error("`file` part of POST form must be a file", 400),
        };
    }

    Response::error("Bad Request", 400)
}
```

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers used in this template) is meant to be executed as compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`workers-rs`](https://github.com/cloudflare/workers-rs) project README.

## Issues

If you have any problems with the `worker` crate, please open an issue on the upstream project issue tracker on the [`workers-rs` repository](https://github.com/cloudflare/workers-rs).

