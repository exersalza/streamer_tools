# Sup
[![wakatime](https://wakatime.com/badge/user/e979c403-8c51-4e2a-8fac-8dea013f7b3b/project/018b6d19-9d0b-421e-ac58-deba931507be.svg)](https://wakatime.com/badge/user/e979c403-8c51-4e2a-8fac-8dea013f7b3b/project/018b6d19-9d0b-421e-ac58-deba931507be)

## Todo
- [ ] persistent websocket/socket timer for subathon
- [ ] crash recovery
- [ ] dynamic timer -> localhost:8080/4aa82db1-6be6-42cb-8931-180fd70daff4
- [ ] timer customization
- [ ] add local time as clock
- [ ] and much much more [todo](./TODO.md)

## Installation
- **First things first DON'T DELETE THE `settings.db`** except you know what you do, in this file is every configuration stored except your twitch bot token.


## Usage
TODO: add usage

## Development
### Deps
- [rust](https://rustup.rs)
- [npm or something like that](https://nodejs.org/en)
- [an sqlite browser](https://www.heidisql.com/)

### to run this project do:

- `git clone https://github.com/exersalza/streamer_tools`
- `cd streamer_tools`
    - `cd frontend && npm i && npm run dev`
- `cargo run --bin server`
And now you should be able to go on `http://localhost:5173` and see a working website
The API is listening on port `80` as the default


### First preview of the webpanel :>
![mockup.png](.assets%2Fmockup.png)
