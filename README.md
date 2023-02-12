Atay2d is a 2D cross platform game engine built in Rust. It is free and open-source forever!

## WARNING

Atay2d is still in the _very_ early stages of development. APIs can and will change (now is the time to make suggestions!). Important features are missing. Documentation is sparse. Please don't build any serious projects in Atay2d unless you are prepared to be broken by API changes constantly.

## Build for Desktop
```sh
cargo r --release
```

## Build for Web (WASM)
```sh
wasm-pack build --release --target web
```
wasm-pack will generate a pkg directory, then you can serve this directory through a web server.
Make you're in the path of this directory using `cd` and run:
```sh
python -m http.server
```
Then you should find the engine is running in the browser through
this [link](http://localhost:8000/).

## License

Atay2d is free, open source and permissively licensed!
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

### Your contributions

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.