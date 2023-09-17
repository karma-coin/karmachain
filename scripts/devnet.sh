#!/usr/bin/env zsh
# This script is meant to be run on Unix/Linux based systems
set -e

echo "Starting dev net"

cargo build --release --features fast-runtime 

run_node() {
    cargo run --release --features fast-runtime  -- --dev \
        --offchain-worker always \
        --rpc-methods unsafe
}

insert_offchain_worker_key() {
    # Make sure that node started 
    sleep 5
    curl --location 'http://localhost:9944/' \
    --header 'Content-Type: application/json' \
    --data '{
        "id": 1,
        "jsonrpc": "2.0",
        "method": "author_insertKey",
        "params": {
            "key_type": "rewa",
            "suri": "//Alice",
            "public": "0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee"
        }
    }'
}

run_node &
insert_offchain_worker_key &

wait

echo "devnet configured..."

