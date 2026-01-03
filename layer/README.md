# Example Layer

This is a very simple Vulkan layer built using `vulkanalia`.
It intercepts the `vkBeginCommandBuffer` and `vkEndCommandBuffer` commands and prints a message before calling the actual command.

## Instructions

To build and enable this layer in the triangle example in this repository, follow these instructions.
They assume that you are starting in the root directory of this Git repository.

1. Build the layer &ndash; `cargo build`
1. Let Vulkan know about the JSON file for your layer that is appropriate for your platform
    - Windows (PowerShell) &ndash; `$env:VK_ADD_LAYER_PATH="$PWD\layer\VK_LAYER_vulkanalia_layer_example.dll.json"`
    - macOS &ndash; `export VK_ADD_LAYER_PATH="$PWD/layer/VK_LAYER_vulkanalia_layer_example.dylib.json"`
    - Linux, BSD, etc. &ndash; `export VK_ADD_LAYER_PATH="$PWD/layer/VK_LAYER_vulkanalia_layer_example.so.json"`
1. Run the triangle example &ndash; `cargo run --bin triangle`
