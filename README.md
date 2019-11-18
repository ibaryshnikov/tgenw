#### Template generator for WebAssembly projects

When you make prototypes with `wasm-bindgen`,
after running `wasm-pack build --target web` there's a `pkg` folder
with a module. Usually it needs a simple web page with some JS code
to load the module. This tool generates such a page.

#### Installing

```
# from local checkout
cargo install --path .
# alternatively, from git
cargo install --git
```

#### Usage

There's no flags, just run

```
tgenw
```

It will produce `index.html` and `main.js`
