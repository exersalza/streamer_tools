# Sup
[![wakatime](https://wakatime.com/badge/user/e979c403-8c51-4e2a-8fac-8dea013f7b3b/project/018b6d19-9d0b-421e-ac58-deba931507be.svg)](https://wakatime.com/badge/user/e979c403-8c51-4e2a-8fac-8dea013f7b3b/project/018b6d19-9d0b-421e-ac58-deba931507be)

## Todo
- [ ] persisten websocket/socket timer for subathon
- [ ] crash recovery
- [x] dynamic timer -> localhost:8080/timer/:id?hours=&minutes=&seconds=
- [ ] timer customization
- [ ] add local time as clock
- [ ] and much much more [todo](./TODO.md)

## Installation
- **First things first DON'T DELETE THE `settings.db`** except you know what you do, in this file is every configuration stored except your twitch token.


## Usage
- Unpack the files into a directory.
- Start the main file
- Now a console should open with a link inside, copy & paste that link into your browser, and a Web panel should be visible.

- timers -> `http://localhost:8080/timer/32?hours=232&minutes=23&seconds=33` use this url to create an timer that looks like `232:23:33`


## Development
### Deps
- `rustup` -> [Rustup](https://rustup.rs/)
- `trunk` -> `cargo install trunk`
- `cargo-watch` -> `cargo install cargo-watch`
- `tailwindcss` -> `npm i -g tailwindcss`
- `wasm` -> `rustup target add wasm32-unknown-unknown`
- `powershell 7.* (only win)` -> [PowerShell 7](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows?view=powershell-7.3) 

### to run this project do:

- `git clone https://github.com/exersalza/streamer_tools`
- `cd streamer_tools`
- (win ps7)`cd frontend && .\build -p 1 && cd ..` to build the frontend
- (unix)`cd frontend && ./build.sh -p 1 && cd ..` to build the frontend
- `cargo run --bin server`
And now you should be able to go on `http://localhost:8080` and see a working website


### First preview of the webpanel :)
![mockup.png](.assets%2Fmockup.png)
