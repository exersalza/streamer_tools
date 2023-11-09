# Sup


## Development
### Deps
- `rustup` -> [Rustup](https://rustup.rs/)
- `trunk` -> `cargo install trunk`
- `wasm` -> `rustup target add wasm32-unknown-unknown`

to run this project do:

- `git clone https://github.com/exersalza/streamer_tools`
- `cd streamer_tools`
- `cargo build`
- `cd frontend && ./build.sh && cd ..` to build the dist folder needed to host the webpanel
- `cargo run --bin server`
And now you should be able to go on `http://localhost:8080` and see a working website
