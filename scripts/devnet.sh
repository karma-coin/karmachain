#!/usr/bin/env zsh
# This script is meant to be run on Unix/Linux based systems
set -e

echo "Starting dev net"

exec cargo run --release -- --dev --verifier --bypass-token="dummy" --auth-dst="https://localhost:8080" --offchain-worker always --rpc-methods unsafe

curl --location 'http://localhost:9944/' \
--header 'Content-Type: application/json' \
--data '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "author_insertKey",
    "params": {
        "key_type": "rewa",
        "suri": "//Alice",
        "public": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
    }
}'

curl --location 'http://localhost:9944/' \
--header 'Content-Type: application/json' \
--data '{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "author_insertKey",
    "params": {
        "key_type": "Veri",
        "suri": "//Alice",
        "public": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
    }
}'

echo "devnet configured..."

