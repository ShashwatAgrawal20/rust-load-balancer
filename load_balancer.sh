exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/rust-load-balancer-target \
    --manifest-path $(dirname $0)/Cargo.toml -- "$@"
