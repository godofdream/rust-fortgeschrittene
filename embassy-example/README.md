# this example is copied from embassy:
https://github.com/embassy-rs/embassy
https://github.com/embassy-rs/embassy/tree/main/examples/stm32f4



Read the embassy getting started guide:

https://embassy.dev/book/#_getting_started


1. Install the toolchain using rustup:

``` console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf # or the specific one for your controller
```

2. Install probe-rs
``` console
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

3. check which chip you have and edit .cargo/config.toml:
``` console
probe-rs chip list
```

4. Change the example to your chip in Cargo.toml

5. Run examples with:
``` console
ls -l src/bin/ # lists examples
cargo run --bin blinky # runs blinky example
```
