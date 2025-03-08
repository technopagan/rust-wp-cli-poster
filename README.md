# rust-wp-cli-poster

Finger practice: make a Rust CLI tool that can post short status messages to my WordPress for microblogging.

## Set it up

``` shell
cp config_example.toml config.toml
# then set the values inside config.toml accordingly
```

## Test it in-flight

``` shell
cargo update && cargo run 'the post message'
```

## Build it properly

``` shell
cargo build --release
strip target/release/rust-wp-cli-poster  
```
