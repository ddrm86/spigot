# Spigot algorithm

## About
Just some sample code to try Rust with WebAssembly.

* Displays the next digit of π every two seconds, with a font size change animation based on CSS transitions.
* Quick and dirty adaptation to Rust of [this implementation](https://mail.python.org/pipermail/edu-sig/2006-July/006810.html) of Gibbon's spigot algorithm.

## Usage
As described in the [Rust and WebAssembly tutorial](https://rustwasm.github.io/docs/book/), run

```
wasm-pack build
```

inside the root folder, and then

```
npm install
npm run start
```

inside the `www` folder. The application should display on http://localhost:8080/

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
