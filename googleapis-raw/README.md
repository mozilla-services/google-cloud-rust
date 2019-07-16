# Generate Rust bindings from `.proto` files

**NOTE:** You do not need to do this. Rust sources have already been generated, but if
you want to regenerate them, follow these steps.

Install protobuf and gRPC compilers:

```
cargo install protobuf-codegen
cargo install grpcio-compiler
```

Pull `.proto` files and generate Rust bindings:

```
git submodule update --init --recursive
./generate.sh spanner
```
