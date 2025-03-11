# Plinth Hello World!
This repo is an example of my Plinth stack. I plan to use this as base for other projects of mine, trying to avoid repeating the same setup code.

The idea is that I have a GPU rendered layer in the background with easy web tech UI floating on top of it. So that I can render massive amounts of data or complex 2D/3D renders without lag and still have the convenience of normal UI when I need it.

This is built on 4 other repos.

[Plinth-Web-Build](https://github.com/gusjengis/Plinth-Web-Build): Pretty nice build and dev server commmands.

[Plinth-Web](https://github.com/gusjengis/Plinth-Web): TSX and CSS that I want to be common between projects.

[Plinth-Util](https://github.com/gusjengis/Plinth-Util): A place to accumulate useful non-application-specific functions. 

[Plinth-Core](https://github.com/gusjengis/Plinth-Core): Nice interface that allows me to use winit and wgpu without having to look at the setup code.

### The Stack
Solid.js(TSX)
wgpu(WASM/Rust)
winit(WASM/Rust)

Solid.js is only included in this repo, and could be easily swapped for any other UI framework. I just happen to want to try it.
