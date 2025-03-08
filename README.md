# rust-wp-cli-poster

Finger practice: make a Rust CLI tool in an afternoon that can post short status messages to my WordPress.

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

## Make it accessible for use

``` shell
ln -s /path/to/rust-wp-cli-poster/target/release/blog /somewhere/in_your_path/blog
chmod +x /somewhere/in_your_path/blog
```

## Use it

``` shell
blog 'Your glorious microblogging message'
```
