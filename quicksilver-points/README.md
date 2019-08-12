WASM Canvas Points
------------------

Prerequisites:
- https://rustwasm.github.io/docs/book/game-of-life/setup.html
- https://github.com/koute/cargo-web#installation

## Build and run

To run it in with WebAssembly do:

```sh
cargo web start
```
Then go to [http://[::1]:8000](http://[::1]:8000) to see it in action.

If you only want to generate the files for building a web release do:
```sh
cargo web deploy --release
```

Also, you can run it in a desktop window with:
```sh
cargo run --release
```
