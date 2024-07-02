# screen
A graphical user interface written in rust. It renders data provided by Sockets

# Description
## C4 
![image](https://github.com/Pending-Name-21/screen/assets/64939916/06636eb9-d45e-43db-8083-ef054437f106)

# Installation
## Requirements
Ensure you have the following packages installed
- curl
- build-essential
- pkg-config
- alsa
- xcb

If you're a Ubuntu user you can try with:

`$ sudo apt install curl build-essential python cmake pkg-config libasound2-dev libxcb-shape0-dev libxcb-xfixes0-dev`

- Vulkan v1.3 up

In order to run Screen you need a device with support for [Vulkan drivers](https://docs.vulkan.org/tutorial/latest/02_Development_environment.html#_linux)

- rust

To install rust please follow this guide:
[Installing rustup on Linux or macOS](https://doc.rust-lang.org/book/ch01-01-installation.html)

Those are Nannou depedencies, you can find more information [here](https://www.guide.nannou.cc/getting_started/platform-specific_setup)

## Build from source
- Clone the repository

`$ git clone --depth=1 --branch=develop https://github.com/Pending-Name-21/screen.git`
- cd into the repository

`$ cd screen`

From here you can use cargo to either build `cargo build` or run `cargo run` you can find more information here:

[Building and Running a Cargo Project](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html?highlight=cargo%20build#building-and-running-a-cargo-project)
