#!/bin/bash

set -e
cd "$(dirname "$0")"

if ! [[ -x "$(command -v grpc_rust_plugin)" ]]; then
    echo "Error: grpc_rust_plugin was not found"
    echo
    echo "To install, run: cargo install grpcio-compiler"
    exit 1
fi

echo "Pulling git submodules"
git submodule update --init --recursive

apis=grpc/third_party/googleapis

proto_files="
grpc/src/proto/grpc/testing/empty.proto
"

for proto in $proto_files; do
    echo "Processing: $proto"
    protoc \
        --rust_out=$PWD/src \
        --grpc_out=$PWD/src \
        --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
        --proto_path=grpc/src/proto/grpc/testing \
        $proto
done

proto_dirs="
spanner/v1
spanner/admin/instance/v1
spanner/admin/database/v1
iam/v1
longrunning
rpc
"

for dir in $proto_dirs; do
    mkdir -p "$PWD/src/$dir"

    for proto in `find $apis/google/$dir/*.proto`; do
        echo "Processing: $proto"
        protoc \
            --rust_out="$PWD/src/$dir" \
            --grpc_out="$PWD/src/$dir" \
            --plugin=protoc-gen-grpc="`which grpc_rust_plugin`" \
            --proto_path="$apis" \
            $proto
    done
done
