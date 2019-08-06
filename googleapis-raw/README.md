# googleapis-raw

These are raw bindings for Google APIs based on [`grpcio`](https://github.com/pingcap/grpc-rs).

## Documentation

To generate and open documentation, run:

```
cargo doc --open
```

## Examples

To run hand-written examples, try:

```
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

## Generating Rust bindings from `.proto` files

**NOTE:** You do not need to do this step. Rust bindings are already included in this repository.

But if you still want to regenerate them from scratch, run:

```
./generate.sh
```

## References

Google APIs and their `.proto` files:

* [Spanner](https://github.com/googleapis/googleapis/tree/master/google/spanner)
* [Bigtable](https://github.com/googleapis/googleapis/tree/master/google/bigtable)
* [Pub/Sub](https://github.com/googleapis/googleapis/tree/master/google/pubsub)

Golang clients:

* [Spanner](https://github.com/googleapis/google-cloud-go/tree/master/spanner)
  ([docs](https://godoc.org/cloud.google.com/go/spanner))
* [Bigtable](https://github.com/googleapis/google-cloud-go/tree/master/bigtable)
  ([docs](https://godoc.org/cloud.google.com/go/bigtable))
* [Pub/Sub](https://github.com/googleapis/google-cloud-go/tree/master/pubsub)
  ([docs](https://godoc.org/cloud.google.com/go/pubsub))
