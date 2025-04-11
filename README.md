<picture style="display: flex; justify-content: center;">
    <img src="public/tux.png" alt="LUG Logo" style="width: 150px; height: auto;">
</picture>

# Linux Users Group
An `HU3120` project.

### Install Requirements
```bash
rustup update # >=1.86.0
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```
This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

### Development
To locally deploy:
```bash
trunk serve --open
```

### Building
To build a Leptos CSR app for release, use the command:
```bash
trunk build --release
```
Refer to [this](https://github.com/splurf/wlrs-wasm) for more verbose instructions on how to deploy for production.