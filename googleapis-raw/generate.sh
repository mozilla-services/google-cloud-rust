#!/bin/bash
#
# Copyright 2020 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

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
bigtable/admin/cluster/v1
bigtable/admin/table/v1
bigtable/admin/v2
bigtable/v1
bigtable/v2
iam/v1
longrunning
pubsub/v1
pubsub/v1beta2
rpc
spanner/admin/database/v1
spanner/admin/instance/v1
spanner/v1
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
