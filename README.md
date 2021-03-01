# Rust Wasm playground (using bindgen, web_sys)

## Install and Run
```bash
# Fetch <- internet
git clone https://github.com/tinmarino/tin_wasm.git && cd tin_wasm.git

# Build -> pkg
wasm-pack build --debug --target web

# Run index.html
python -m http.server
firefox http://0.0.0.0:8000/
```

## "Play"

* __translate__: wasd (and rf)
* __rotate__: arrow keys

![Screenshot image not found](res/screenshot_1.png "Demo")

## Dev

```
├── index.html            <- 1
├── src/
    ├── lib.rs            <- 2
    ├── game.rs           <- 3
    ├── camera.rs         <- 5
    ├── util.rs           <- 4
    ├── constants.rs

├── Cargo.toml
├── README.md
├── res/
├── pkg/
├── target/
```
