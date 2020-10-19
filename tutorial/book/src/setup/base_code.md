# Base code

**Code:** [main.rs](https://github.com/KyleMayes/vulkanalia/tree/master/tutorial/src/00_base_code.rs)

In the `Development environment` chapter we created a Cargo project and added the necessary dependencies. In this chapter we will be replacing the code in the `src/main.rs` file with the following code:

```rust
use anyhow::*;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() -> Result<()> {
    pretty_env_logger::init();
    info!("Starting program...");

    // Window

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = App::create(&window)?;
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying => app.render(&window).unwrap(),
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                app.destroy();
            }
            _ => {}
        }
    });
}

/// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {}

impl App {
    /// Creates our Vulkan app.
    fn create(window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    /// Renders a frame for our Vulkan app.
    fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    fn destroy(&mut self) {}
}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {}
```

We first import `anyhow::Result` which will be used to easily represent all of the results in our program. Next we import all of the `winit` types we need to create a window and start an event loop for that window.

Next comes our `main` function (which returns the imported `anyhow::Result` type). This function starts by initializing `pretty_env_logger` which will print our logs to the console (as shown later).

Then we create an event loop and window to render to using `winit` using `LogicalSize` which will scale the window according to the DPI of your display. If you want to know more about UI scaling you can read the [relevant `winit` documentation](https://docs.rs/winit/latest/winit/dpi/index.html).

Next we create an instance of our Vulkan app (`App`) and enter into our rendering loop. This loop will continually render our scene to the window until you request the window to be closed at which point the app will be destroyed and the program will exit. The `destroying` flag is necessary to not keep attempting to render the scene while the app is being destroyed which would most likely result in the program crashing after attempting to access Vulkan resources that have been destroyed.

Lastly comes `App` and `AppData`. `App` will be used to implement the setup, rendering, and destruction logic required for the Vulkan program we will be building over the course of the following chapters. `AppData` will serve simply as a container for the large number of Vulkan resources we will need to create and initialize which will allow for them to be easily passed to functions to be read and/or modified.

This will come in handy because many of the following chapters consist of adding a function which takes a `&mut AppData` and creates and initializes Vulkan resources. These functions will then be called from our `App::create` constructor method to set up our Vulkan app. Then, before our program exits, these Vulkan resources will be released by our `App::destroy` method.

## Resource management

Just like each chunk of memory allocated in C with `malloc` requires a corresponding call to `free`, every Vulkan object that we create needs to be explicitly destroyed when we no longer need it. In Rust it is possible to perform automatic resource management using [RAII](https://en.wikipedia.org/wiki/Resource_Acquisition_Is_Initialization) possibly combined with smart pointers like `Rc` or `Arc`. However, the author of <https://vulkan-tutorial.com> chose to be explicit about allocation and deallocation of Vulkan objects in this tutorial and I have decided to take the same approach. After all, Vulkan's niche is to be explicit about every operation to avoid mistakes, so it's good to be explicit about the lifetime of objects to learn how the API works.

After following this tutorial, you could implement automatic resource management by writing Rust structs that wrap Vulkan objects and release them in their `Drop` implementation. RAII is the recommended model for larger Vulkan programs, but for learning purposes it's always good to know what's going on behind the scenes.

Vulkan objects are either created directly with commands like `vkCreateXXX`, or allocated through another object with commands like `vkAllocateXXX`. After making sure that an object is no longer used anywhere, you need to destroy it with the counterparts `vkDestroyXXX` and `vkFreeXXX`. The parameters for these commands generally vary for different types of objects, but there is one parameter that they all share: `pAllocator`. This is an optional parameter that allows you to specify callbacks for a custom memory allocator. We will ignore this parameter in the tutorial and always pass `None` as argument.
