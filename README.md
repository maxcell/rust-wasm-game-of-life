# Rust + WASM Conway's Game of Life

Here is my implementation following the [Rust + WASM book](https://rustwasm.github.io/docs/book/introduction.html).
This goes through the first half of the section for creating the life game, so still using the `pre` before putting
as a canvas.

### How to Read this Repository

Primarily the pieces of work that are being done are happening in `src/lib.rs` and `www/index.js`. Before you can
use this project, you need to be sure to `cargo install` so you can have the correct dependencies. Before continuing, you must also have
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.

Then in the directory, you can run `wasm-pack build` which should create a `pkg/`, this is how we are able to connect
our own package to be consumed by the browser. From there you can move into the `www` directory (`cd www/`) and run
`npm run start` and it should be running <http://localhost:8080>.

### Notes about errors caught on stream

During the stream, we got through all the steps and we encountered a HUGE output. Turns out the output was composed of
two things:

1. The `.wat` output
2. An error with `wasm-opt`

This is the error output I saw:

```shell
[wasm-validator error in module] unexpected true: Exported global cannot be mutable, on 
global$0
Fatal: error in validating input
Error: failed to execute `wasm-opt`: exited with exit code: 1
  full command: "/Users/<current_user>/Library/Caches/.wasm-pack/wasm-opt-a528729925722b63/wasm-opt" "/<path_to_project>/wasm-game-of-life/pkg/wasm_game_of_life_bg.wasm" "-o" "/<path_to_project>/wasm-game-of-life/pkg/wasm_game_of_life_bg.wasm-opt.wasm" "-O"
To disable `wasm-opt`, add `wasm-opt = false` to your package metadata in your `Cargo.toml`.
```

There was a comment on a [related issue](https://github.com/rustwasm/wasm-pack/issues/886#issuecomment-667669802) that solved it for me.

Adding this to `Cargo.toml`:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz", "--enable-mutable-globals"]
```