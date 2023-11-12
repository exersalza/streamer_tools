# Sup


## Installation
- **First things first DON'T DELETE THE `settings.db`** except you know what you do, in this file is every configuration stored except your twitch token.

## Development
### Deps
- `rustup` -> [Rustup](https://rustup.rs/)
- `trunk` -> `cargo install trunk`
- `wasm` -> `rustup target add wasm32-unknown-unknown`
- `powershell 7.* (only win)` -> [PowerShell 7](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows?view=powershell-7.3) 

### to run this project do:

- `git clone https://github.com/exersalza/streamer_tools`
- `cd streamer_tools`
- (win)`cd frontend && .\build -prod 1 && cd ..`  to build the frontend
- ~~(unix)`cd frontend && ./build.sh && cd ..` to build the frontend~~ (not up to date with ps1 script)
- `cargo run --bin server`
And now you should be able to go on `http://localhost:8080` and see a working website


### First preview of the webpanel :>
![mockup.png](.assets%2Fmockup.png)