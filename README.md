# custom-p2p-protocol
This is an example implementation of a p2p protocol based on https://en.bitcoin.it/wiki/Protocol_documentation .

# Prerequisites
For minimal library dependency, this implementation runs with unstable features. For this, nightly channel is needed for compilation:
```
rustup install nightly
rustup default nightly
```

## Run in debug mode:
```
cargo run
```

## Run in release mode:
```
cargo run --release
```