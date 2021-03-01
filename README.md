# Rust Wasm playground (using bindgen, web_sys)

```bash
wasm-pack build --debug --target web
python -m http.server
```

```bash
git clone https://github.com/tinmarino/tin_wasm.git && cd tin_wasm.git
```

# References:

* Javascript: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial
* Bindgen structures: https://rustwasm.github.io/wasm-bindgen/examples/webgl.html

# dump
// the trait bound `T: FromWasmAbi` is not satisfied: the trait `FromWasmAbi` is not implemented for `T`
// the parameter type `impl FnMut(T)` may not live long enough: ...so that the type `impl FnMut(T)` will meet its required lifetime bounds
// expected a `Fn<(T,)>` closure, found `impl FnMut(T) + 'static`: expected an `Fn<(T,)>` closure, found `impl FnMut(T) + 'static`

# TODO

* Vim, better rust support (fold, jump, doc, completion with keystrokes)
* Create my constans.rs with PI and keystrokes
* Create other HelloWorld Project with hardcoded matrix where possible
* Move view <- webgl water
* Landon, fix meshiewer
