WASM 2D experiments
===================

This is a collection of experiments with 2D graphics libraries which are compilable to WASM.

## Libraries

* [wasm-canvas-points](./wasm-canvas-points): Drawing circles into web canvas with [piet](https://github.com/linebender/piet)
* [raqote-points](./raqote-points): Drawing circles into web canvas with [raqote](https://github.com/jrmuizel/raqote)
* [quicksilver-points](./quicksilver-points/): Drawing circles into OpenGL canvas with [Quicksilver](https://ryanisaacg.com/quicksilver/) and [Lyon](https://github.com/nical/lyon)

## Performance

I'm implementing a microbenchmark [Drawing Circular Points](https://www.desultoryquest.com/blog/drawing-anti-aliased-circular-points-using-opengl-slash-webgl/) with these libraries and measure
FPS a on my notebook: [measurements.md](./measurements.md)

## Articles (2019)

* [2D Graphics on Modern GPU](https://raphlinus.github.io/rust/graphics/gpu/2019/05/08/modern-2d.html)
* [A look at pathfinder](https://nical.github.io/posts/a-look-at-pathfinder.html)
* [A Guide to Rust Graphics Libraries](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019) (3D)
