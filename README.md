# Google Cloud Rust Client

This repository contains [`Rust`](https://www.rust-lang.org/) client libraries to interact with various [`Google Cloud Platform`](https://cloud.google.com/) services.

*Note:* This library currently uses the following dependencies. ***THESE DEPENDENCIES MUST MATCH WITH THE CALLING CODE*** As always, be sure to consult the `Cargo.toml` to verify the versions being used by this iteration of the library.

* `protobuf={version="=2.28.0"}`
* `grpcio={version="0.13.0"}`

## Disclaimers

### Currently in production:

[`Cloud Spanner`](https://cloud.google.com/spanner) (Horizontally scalable relational database)

### Auto-generated:

[`Cloud BigTable`](https://cloud.google.com/bigtable) (Petabyte-scale, low-latency, NoSQL non-relational database)

[`Cloud Pub/Sub`](https://cloud.google.com/pubsub) (Messaging and ingestion for event-driven systems and streaming analytics)

[`Cloud Storage`](https://cloud.google.com/storage) (Multi-class, multi-region, RESTful object storeage)

**NOTE: These generated clients are under development and should be considered
experimental!**

This library is currently locked to grpcio 0.12.0 and protobuf 2.28.0. This is due to
an [internal dependency of grpcio](https://github.com/tikv/grpc-rs/issues/584). Be sure that you match
these library versions in your library or application.

## Usage

## Rust bindings for Google APIs

See [`googleapis-raw`](googleapis-raw/examples) for raw bindings based on
[`grpcio`](https://github.com/pingcap/grpc-rs).

See [`googleapis`](googleapis/examples) for high-level bindings (not ready for use yet).

## Contributing

Contributions to this library are always welcome and highly encouraged.

See [CONTRIBUTING](CONTRIBUTING.md) for more information on how to get started.

Please note that this project is released with a Contributor Code of Conduct. By participating in this project you agree to abide by its terms.

## License

Apache 2.0 - See [LICENSE](LICENSE.md) for more information.

## Disclaimer

This is not an officially supported Google product, but was initially created via an agreement between [`Mozilla`](https://www.mozilla.org/), [`Google Cloud`](https://cloud.google.com/), [`Ferrous Systems`](https://ferrous-systems.com/), and [`IGNW`](https://www.ignw.io/).

Thank you
