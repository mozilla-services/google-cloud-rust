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

# *NOTE*: Make sure to update cargo plugins after protobuf updates
#  cargo install grpcio-compiler
#  cargo install protobuf-codegen
# May need to delete the ./src/*/[^(mod)].rs to force regeneration of files
# (deleting the mod.rs files will require adding `pub (crate)mod crate::*`
# cross referencing.)
set -e
cd "$(dirname "$0")"

## remove old files:
echo "Purging old files..."
find src -name "*.rs" -and -not \( -name "mod.rs" -or -name "lib.rs" \) -print -delete

## updating plugins
echo "Updating cargo..."
cargo update
echo "Updating plugin..."
# Lock on 2.28.0 until grpcio supports 3+
#cargo install protobuf-codegen --version 2.28.0
cargo install protobuf-codegen

if ! [[ -x "$(command -v grpc_rust_plugin)" ]]; then
    echo "Error: grpc_rust_plugin was not found"
    echo
    echo "To install, run: cargo install grpcio-compiler"
    exit 1
fi

echo "Pulling git submodules"
# comment out to work on master...
# git submodule update --init --recursive

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

#storage_dirs="
#storage/v1
#"

# Big table has dependencies on "ruby_package"
big_table_dirs="
bigtable/admin/v2
bigtable/v2
"

# Spanner specific dependencies
spanner_dirs="
spanner/admin/database/v1
spanner/admin/instance/v1
spanner/v1
"

# The following directories should correspond with the ones defined under
# `grpc/third_party/googleapis/google` and contain the various, required `.proto`
# definition files that should be read and generated from.
# *NOTE:* These often require some manual editing, since there's no way to automatically
# determine what to use. Basically, when you generate, you may see some undefined
# items and you get to go hunting for what they might be. (Often, looking at another
# library can help determine paths.)
proto_dirs="
api
api/servicecontrol/v1
api/servicemanagement/v1
cloud/asset/v1
cloud/orgpolicy/v1
cloud/orgpolicy/v2
cloud/osconfig/v1
cloud/osconfig/v1alpha
cloud/osconfig/agentendpoint/v1
logging/type
type
iam/v1
identity/accesscontextmanager/v1
identity/accesscontextmanager/type
longrunning
pubsub/v1
pubsub/v1beta2
rpc
$big_table_dirs
$storage_dirs
$spanner_dirs
"

# obsolete proto:
# cloud/orgpolicy/v1
# cloud/osconfig/v1
# logging
# identity/accesscontextmanager/v1
# identity/accesscontextmanager/type


# The following are required to support Spanner only
reduced_proto_dirs="
iam/v1
longrunning
rpc
spanner/admin/database/v1
spanner/admin/instance/v1
spanner/v1
"
SRC_ROOT=$PWD


for dir in $proto_dirs; do
    mkdir -p "$SRC_ROOT/src/$dir"
    echo "Processing: $dir..."

    for proto in `find $apis/google/$dir/*.proto`; do
        echo "Processing: $proto ..."
        protoc \
            --rust_out="$SRC_ROOT/src/$dir" \
            --grpc_out="$SRC_ROOT/src/$dir" \
            --plugin=protoc-gen-grpc="`which grpc_rust_plugin`" \
            --proto_path="$apis:grpc/third_party/upb:grpc/third_party/protobuf/src/:" \
            $proto
    done
done

echo "Make sure you generate the mod.rs files!"

# under `../tools/mod_updater` is a python script that can automatically try to genereate the
# mod files. It's not perfect, but it works. You'll still need to add some of the foreign references
# that can crop up during builds. (e.g. if you get missing references to `empty`, you need to add
# `pub(crate) use crate::empty;` to the `mod.rs` file. Finding other references can be a bit of a
# challenge, but it's possible.)

# ls -1 --color=never . |grep -v mod |sed "s/\.rs//" |sed "s/^/pub mod /" | sed "s/$/;/" > mod.rs \; --print
# echo "pub(crate) use crate::empty;" >> */v1/mod.rs
