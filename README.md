# Rust + WASM Conway's Game of Life

Here is my implementation following the [Rust + WASM book](https://rustwasm.github.io/docs/book/introduction.html).
This goes through the first half of the section for creating the life game, so still using the `pre` before putting
as a canvas.

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