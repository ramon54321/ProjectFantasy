<div align="center">
    <span><img src="https://www.vulkan.org/user/themes/vulkan/images/logo/vulkan-logo.svg" width="400"></span>
    <span><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1920px-Rust_programming_language_black_logo.svg.png" width="100"></span>
</div>

## Map Generator powered by Vulkan

This project is mainly focused on the learning process of using [Vulkan API](https://www.vulkan.org/) with [Rust](https://www.rust-lang.org/). The goal is to explore techniques of abstracting the raw Vulkan API exposed by [Vulkano](http://vulkano.rs/) through the development of a simple map generator.

### Installation

Build and run the Vulkan backed window with:

```
cargo run
```

Remember to recompile shaders if changes are made to them.

```
./compile_shaders.sh
```

This will compile `.vert` and `.frag` files into Vulkan `.spv` SPIR-V bytecode, which will be read by the main application.

### Vulkan Wrapping Architecture

Currently the architecture of the core Vulkan components are layed out as follows:

<img src="https://github.com/ramon54321/ProjectFantasy/blob/main/docs/vulkan_arch.png?raw=true" width="800">

The diagram shows the dependencies between the components. The arrows can be read as "is dependent on". For example `Device` is dependent on `Surface`. In addition, the macro components `Base`, `Fixture` and `Sweep` also have dependencies amongst one another. `Sweep` is dependent on `Fixture`, which is itself dependent on `Base`. If any component within a macro component changes or gets updated, all macro components which depend on it needs to be rebuilt. For example, if the `SwapChain` is rebuilt, for example due to a window resize, the `Fixture` and all `Sweeps` dependent on said `Fixture`, need to be rebuilt on the new `Fixture`
