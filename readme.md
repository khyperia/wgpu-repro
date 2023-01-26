To repro:

1) `wasm-pack build --target web --debug`
2) `python server.py`
3) open `localhost:8080` in a chrome canary that has the enable-unsafe-wgpu flag enabled: `chrome://flags/#enable-unsafe-webgpu`
