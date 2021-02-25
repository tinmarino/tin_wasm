# Rust Wasm playground (using bindgen, web_sys)

```bash
wasm-pack build --target web
python -m http.server
```

```bash
git clone https://github.com/tinmarino/tin_wasm.git && cd tin_wasm.git
```

# References:

* Javascript: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial
* Bindgen structures: https://rustwasm.github.io/wasm-bindgen/examples/webgl.html

# TODO

* Create other HelloWorld Project with hardcoded matrix where possible
* Move view <- webgl water
* Landon, fix meshiewer
