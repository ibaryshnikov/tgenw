#### Template generator for WebAssembly projects

When you make prototypes with `wasm-bindgen`,
after running `wasm-pack build --target web` there's a `pkg` folder
with a module. Usually it needs a simple web page with some JS code
to load the module. This tool generates such a page.

#### Installing

```bash
# from local checkout
cargo install --path .
# alternatively, from git
cargo install --git https://github.com/ibaryshnikov/tgenw
```

#### Usage

There're no flags, just run

```bash
tgenw
```

it will produce `index.html` and `main.js`
