<div align="center">
    <span><img src="https://www.vulkan.org/user/themes/vulkan/images/logo/vulkan-logo.svg" width="400"></span>
    <span><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1920px-Rust_programming_language_black_logo.svg.png" width="100"></span>
</div>

## Ray Marching powered by Vulkan

Small minimal demo showcasing ray marching as described by Michael Walczky in his [blog post](https://michaelwalczyk.com/blog-ray-marching.html). Graphics card communication is handled through the [Vulkan API](https://www.vulkan.org/). My [minimal demo of Vulkan](https://github.com/ramon54321/ProjectVulkan) is an example of how Vulkan can be used with [Rust](https://www.rust-lang.org/).

### Installation

Build and run the Vulkan backed window with:

```
cargo run
```

In a separate terminal run the shader compile watcher with:

```
./compile_shaders_watch.sh
```

This will ensure any changes to `.vert` and `.frag` files will be automatically compiled into Vulkan `.spv` SPIR-V bytecode, which will be read by the main application.

The application supports live reload of shaders by clicking anywhere in the viewport.
