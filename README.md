# Rust + WASM snowfall animation

This repo contains code and binaries for a simple snowfall animation I made with Rust + WASM.
For a live demo see [here](https://bazizi.github.io/rust-wasm-snow-animation/index.html).

## Build instructions
- To build the rust source locally, download [wasm-pack](https://github.com/rustwasm/wasm-pack) and run:
```sh
wasm-pack build -t web
```

- Run an HTTP server from the root of the repo. Example using Python:

```sh
python -m http.server
```

- Open [http://localhost:8000/](http://localhost:8000/) in a web browser
