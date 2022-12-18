# Do Not Enter

Most code is adapted from [rust-raspberrypi-OS-tutorials][tutorials], under the MIT or Apache-2.0 license, at your discretion.

The following, until specified, is copied from [rust-raspberrypi-OS-tutorials][tutorials], and adapted where necessary

## 🛠 System Requirements

Building and deploying is primarily targeted at **Linux**-based distributions. If on Windows, WSL is strongly recommended, using [usbipd](https://github.com/dorssel/usbipd-win) to connect to the physical hardware.

If developing on Windows, you are on your own...

### 🚀 The tl;dr Version

1. [Install Docker Engine][install_docker].
1. (**Linux only**) Ensure your user account is in the [docker group].
1. Prepare the `Rust` toolchain. Most of it will be handled on first use through the
   [rust-toolchain](rust-toolchain.toml) file. What's left for us to do is:

   1. If you need to install Rust from scratch:

      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

      source $HOME/.cargo/env
      ```

   1. With Rust installed

      ```bash
      cargo install cargo-binutils rustfilt
      ```

1. In case you use `Visual Studio Code`, I strongly recommend installing the [Rust Analyzer extension].
1. (**macOS only**) Install a few `Ruby` gems.

This was last tested by the author with Ruby version `3.0.2` on `macOS Monterey`. If you are using
`rbenv`, the respective `.ruby-version` file is already in place. If you never heard of `rbenv`,
try using [this little guide](https://stackoverflow.com/a/68118750).

Run this in the repository root folder:

```bash
bundle config set --local path '.vendor/bundle'
bundle config set --local without 'development'
bundle install
```

[docker group]: https://docs.docker.com/engine/install/linux-postinstall/
[rust analyzer extension]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer

### 🧰 More Details: Eliminating Toolchain Hassle

This series tries to put a strong focus on user friendliness. Therefore, efforts were made to
eliminate the biggest painpoint in embedded development as much as possible: `Toolchain hassle`.

Rust itself is already helping a lot in that regard, because it has built-in support for
cross-compilation. All that we need for cross-compiling from an `x86` host to the Raspberry Pi's
`AArch64` architecture will be automatically installed by `rustup`. However, besides the Rust
compiler, we will use some more tools. Among others:

- `QEMU` to emulate our kernel on the host system.
- A self-made tool called `Minipush` to load a kernel onto the Raspberry Pi on-demand over `UART`.
- `OpenOCD` and `GDB` for debugging on the target.

There is a lot that can go wrong while installing and/or compiling the correct version of each tool
on your host machine. For example, your distribution might not provide the latest version that is
needed. Or you are missing some hard-to-get dependencies for the compilation of one of these tools.

This is why we will make use of [Docker][install_docker] whenever possible. We are providing an
accompanying container that has all the needed tools or dependencies pre-installed, and it gets
pulled in automagically once it is needed. If you want to know more about Docker and peek at the
provided container, please refer to the repository's [docker](docker) folder.

[install_docker]: https://docs.docker.com/engine/install/#server

## 📟 USB Serial Output

Since the kernel developed in the tutorials runs on the real hardware, it is highly recommended to
get a USB serial cable to get the full experience.

- You can find USB-to-serial cables that should work right away at [\[1\]] [\[2\]], but many others
  will work too. Ideally, your cable is based on the `CP2102` chip.
- You connect it to `GND` and GPIO pins `14/15` as shown below.
- [Tutorial 5](05_drivers_gpio_uart) is the first where you can use it. Check it out for
  instructions on how to prepare the SD card to boot your self-made kernel from it.
- Starting with [tutorial 6](06_uart_chainloader), booting kernels on your Raspberry is getting
  _really_ comfortable. In this tutorial, a so-called `chainloader` is developed, which will be the
  last file you need to manually copy on the SD card for a while. It will enable you to load the
  tutorial kernels during boot on demand over `UART`.

![UART wiring diagram](doc/wiring.png)

[\[1\]]: https://www.amazon.de/dp/B0757FQ5CX/ref=cm_sw_r_tw_dp_U_x_ozGRDbVTJAG4Q
[\[2\]]: https://www.adafruit.com/product/954
[tutorials]: https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
