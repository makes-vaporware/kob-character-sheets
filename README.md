#kob-character-sheets

Interactive character sheet for "Kids On Bikes"-style TTRPGs. In early development.

Run:

1. `cargo build` to build the web app,
2. you may also need to run `rustup target add wasm32-unknown-unknown` so that Rust can compile the code to WebAssembly,
3. `trunk serve --open` to open the app in your default browser.
