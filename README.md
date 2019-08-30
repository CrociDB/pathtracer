An implementation based on `Ray Tracing in One Weekend` by **Peter Shirley**[1] in Rust with CPU multithreading.

[1] http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf

![Pathtracer](https://i.imgur.com/a2ws5oV.png)

## Building

### Linux (Debian based)

Assuming you have `rustc 1.33` or newer already installed, install `libsdl2-dev` on your system:

```sh
$ sudo apt-get libsdl2-dev
```

Open `Cargo.toml` and remove `build` property (that's only for Windows).

Now simply go for:

``` sh
$ cargo run --release
```

*I strongly recommend running it on the release configuration, since it's optimized and runs at least 15x faster.*

### Windows (MSVC)

Assuming you have `rustc 1.33` or newer already installed, download SDL2 runtime libraries from [here](https://www.libsdl.org/download-2.0.php), then extract all `*.lib` files to `pathtracer-root\msvc\lib\64\*.lib` and all the `*.dll` files to `pathtracer-root\msvc\dll\64\*.lib`, assuming you're running on a `x86-64` environment. Otherwise, open `src\build.rs` and figure it out.

Now simply do:

```
> cargo run --release
```

*I strongly recommend running it on the release configuration, since it's optimized and runs at least 15x faster.*

## Instructions

When a black window open, press `SPACE` and wait sometime for it to render.

You can change how many rays per pixel you want to cast, on `src/tracer/mod.rs` there's a global constant called `SAMPLINGS_PER_PIXEL`, it's set to `100` by default, but you can test how many rays you want.

*Warning: be careful when running more than **100** rays per pixel, since it may get your computer really slow for a few minutes.*
