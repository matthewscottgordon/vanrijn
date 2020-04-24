![](https://github.com/matthewscottgordon/vanrijn/workflows/Rust/badge.svg)

# Vanrijn

Vanrijn is a [physically based](https://en.wikipedia.org/wiki/Physically_based_rendering)
[ray tracer](https://en.wikipedia.org/wiki/Ray_tracing_(graphics)). Many thanks to the
authors of the book
["Physically Based Rendering: From Theory to Implementation](https://www.pbrt.org/) from
which many of the algorithms used here are taken. This is, however _not_ a Rust port of
the C++ PBRT rederer described in that book.

This crate is structured as a library; main.rs is just a glorified test harness which
shows an example of using the library to render a scene. It uses SDL2 to display the
rendered image.

On Ubuntu 19.04, if you have the libsdl2-dev package installed you should be able to
run "cargo run" and see a window with a test scene rendered into it. In theory it should
work on any platform with SDL2 installed but I've only tested it on Ubuntu Linux.

![](.github/output.png?raw=true "Test Image 1")
![](.github/output2.png?raw=true "Test Image")
