# googleapis-raw

These are raw bindings for Google APIs based on [`grpcio`](https://github.com/pingcap/grpc-rs).

## Version notes

This library is currently locked to grpcio 0.13.0 and protobuf 2.28.0.  This is due to
an [internal dependency of grpcio](https://github.com/tikv/grpc-rs/issues/584). Be sure that you match
these library versions in your library or application.

## Documentation

To generate and open documentation, run:

```sh
cargo doc --open
```

## Examples

To run hand-written examples, try:

```sh
cargo run --example spanner-query
cargo run --example bigtable-query
```

## Setting up Google Cloud SDK

Before running examples, make sure the Google Cloud SDK is set up in your environment.
If you need help, follow these guides:

1. [Installing the SDK](https://cloud.google.com/sdk/install)
2. [Setting up the SDK](https://cloud.google.com/sdk/docs/initializing)
3. [Getting started with Authentication](https://cloud.google.com/docs/authentication/getting-started)

As a final check:

* Run `gcloud info` to see the SDK configuration.
* Run `echo $GOOGLE_APPLICATION_CREDENTIALS` to verify that the credentials have been set up.
* Run `gcloud auth login` to login into Google Cloud

There is Docker setup available that installs all necessary tools, libraries, see the [README](../docker/README.md)
inside the `./docker` folder.

Useful links for setting up specific Google services:

* [Setting up Spanner](https://cloud.google.com/spanner/docs/getting-started/set-up)
* [Installing the Cloud SDK for Cloud Bigtable](https://cloud.google.com/bigtable/docs/installing-cloud-sdk)
* [Quickstart using the Pub/Sub CLI tool](https://cloud.google.com/pubsub/docs/quickstart-cli)

## Generating Rust bindings from `.proto` files

**NOTE:** You may need to update these bindings after protobuf updates.
The process requires an update to the `protobuf-codegen` cargo plugin.

This requires the installation of [protobuf](https://google.github.io/proto-lens/installing-protoc.html) library
and [protoc-gen-rust](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen), a plugin
for protobuf. The installed protobuf version and the protobuf crate should have the same version, e.g. `2.7.0`.
Installation of the protoc-gen-rust plugin is done via `cargo`:

```sh
cargo install protobuf-codegen
```

Make sure the `protoc-gen-rust` binary is available in your `$PATH` env variable.

Then:

1) In the `./src` directory, remove all the `*.rs` that are not
`mod.rs` [1]. You may wish to run the `prepare.sh` script.

2) Run the `./generate.sh` script

ensure that a proper build works by running `cargo build`.

_[1] The `generate.sh` script will NOT generate the required `mod.rs` files for the directories. In addition, the generated rust modules will look for several modules that are `super` to their package. `protoc` may not overwrite an existing, generated rust file which could lead to complications. It's easiest if you simily leave the `mod.rs` files in place and remove the other rust files, or copy the `mod.rs` files from a backup. Running a `cargo build` will definitely identify the modules that may be missing and that you'll have to add via a line like:_

```rust
pub(crate) use crate::{
    empty,
    iam::v1::{iam_policy, policy},
    longrunning::operations,
};
```

_(which was taken from `src/rpc/spanner/admin/instance/v1/mod.rs`)_

Please note that the source grpc repo may contiain one or more older submodule references that may
need to be updated (e.g. `.grpc/third_party/googleapis`). This may require manual updating as well
as manunal updating/correction of the various, `mod.rs` files. Pay close attention to interdependencies.
These may need to be specified in the directory mods (see `.src/storage/v1/mod.rs` for an example.
Remember, you can use the `r#` prefix in rust to except a reserved name for use, e.g.
`use mod crate::r#type`)

## Monitoring for changes

Google will introduce changes and modifications to the GRPC interface for spanner, bigtable and other service. You are STRONGLY encouraged to monitor the [project release notes](https://cloud.google.com/release-notes/all) (particularly for [Spanner](https://cloud.google.com/spanner/docs/release-notes) and [BigTable](https://cloud.google.com/bigtable/docs/release-notes), as well as any other service this library should start to support). RSS feeds are available for these release notes, and are listed on their respective pages.

### When Updating

When initializing the submodule dependencies, you can use

`git submodule update --init --recursive`

to do a targeted update of the grpc code:

```bash
git submodule update --remote grpc
pushd grpc/third_party
git submodule update --remote protobuf
git submodule update --remote googleapis
popd
```

to pull a version of the libraries you require. Unfortunately, I have found that `git submodule update` does **NOT** always properly update dependencies to the latest master/main version.
Be sure that the `grpc/third_party/googleapis` submodule is updated at least once per quarter. This directory contains the Protobuf declarations and can often contain undisclosed changes that need to be reflected into the generated code.

**NOTE**: this may alter the pre-generated mod files requiring old
modules to be dropped or new modules to be added. Ensure that the
various `mod.rs` files capture these changes.

## Google Cloud Console

Links to Google Cloud Console for our testing environment:

* [Spanner Console](https://console.cloud.google.com/spanner/instances?project=mozilla-rust-sdk-dev)
* [Bigtable Console](https://console.cloud.google.com/bigtable/instances?project=mozilla-rust-sdk-dev)
* [Pub/Sub Console](https://console.cloud.google.com/cloudpubsub/topic/detail/mytopic?project=mozilla-rust-sdk-dev)

## References

Google APIs and their `.proto` files:

* [Spanner](https://github.com/googleapis/googleapis/tree/master/google/spanner)
* [Bigtable](https://github.com/googleapis/googleapis/tree/master/google/bigtable)
* [Pub/Sub](https://github.com/googleapis/googleapis/tree/master/google/pubsub)

Golang clients:

* [Spanner client](https://github.com/googleapis/google-cloud-go/tree/master/spanner)
  ([docs](https://godoc.org/cloud.google.com/go/spanner))
* [Bigtable client](https://github.com/googleapis/google-cloud-go/tree/master/bigtable)
  ([docs](https://godoc.org/cloud.google.com/go/bigtable))
* [Pub/Sub client](https://github.com/googleapis/google-cloud-go/tree/master/pubsub)
  ([docs](https://godoc.org/cloud.google.com/go/pubsub))
