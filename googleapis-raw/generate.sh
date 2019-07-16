#!/bin/bash

set -e
cd "$(dirname "$0")"

if [[ $# -eq 0 ]] ; then
    echo "Use at least one of the following arguments:"
    echo "  - spanner"
    exit 1
fi

for service in "$@"; do
    case "$service" in
        spanner)
            version=v1
            dir=$PWD/src/$service/$version
            mkdir -p $dir

            for proto in `find grpc/third_party/googleapis/google/$service/$version/*.proto`; do
                protoc \
                    --rust_out=$dir \
                    --grpc_out=$dir \
                    --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
                    --proto_path=grpc/third_party/googleapis \
                    $proto
            done

            protoc \
                --rust_out=$dir \
                --grpc_out=$dir \
                --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
                --proto_path=grpc/src/proto/grpc/testing \
                grpc/src/proto/grpc/testing/empty.proto
            ;;

        *)
            echo "Unrecognized argument: $service"
            exit 1
            ;;
    esac

    echo "Bindings written into: $dir"
done
