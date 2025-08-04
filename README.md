This is small example on how to use bliss-rs with a WASM target.

Following those steps should generate a .js file that can be started with e.g.
node.

How to build:
1. Install emscripten and activate it using the
[offical instructions](https://emscripten.org/docs/getting_started/downloads.html#installation-instructions-using-the-emsdk-recommended)
2. Install the wasm32-unknown-emscripten target using:
```
    rustup target add wasm32-unknown-emscripten
    rustup install nightly
    rustup component add rust-src --toolchain nightly
```

3. Build the .js file using `cargo +nightly build --release --target=wasm32-unknown-emscripten -Z build-std=panic_abort,std`
4. Start the js file with `node ./target/wasm32-unknown-emscripten/release/wasm-test.js` to test it out!
