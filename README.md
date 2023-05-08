## Project board
Tasks for the current prototype are here:
https://github.com/orgs/carrot-spillage/projects/1/views/1


Start the native app: cargo run
Start the web build: trunk serve
requires trunk: cargo install --locked trunk
requires wasm32-unknown-unknown target: rustup target add wasm32-unknown-unknown
this will serve your app on 8080 and automatically rebuild + reload it after code changes