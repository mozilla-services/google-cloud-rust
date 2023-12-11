# Notable changes

Much of this library is auto-generated, so it would not make sense to list all of the changes between each version.
However, if there are significant changes that may impact code, I will try to note them here.

## 0.16.1

* Update to the underlying gprc libraries to use latest version as of
  19 Oct 2023

* Expose `VERSION:&str` which indicates the crate version of this
  library.

## 0.14.0

### Logins

It appears that the GRPCIO library changed regarding logging in.

Previously, connections to GCP would be done by

```rust
 let channel = ChannelBuilder::new(Arc::clone(env))
        // Set the max size to correspond to server-side limits.
        .max_send_message_len(1 << 28)
        .max_receive_message_len(1 << 28)
        .secure_connect(&endpoint, creds);
```

The new connection system appears to have dropped the `secure_connect()` function in favor of doing the following:

In Cargo.toml, you will need to specify the `_secure` feature for `grpcio`

```toml
grpcio = {version="0.12.0" features=["_secure"]}
```

In your connection code, you can then use the following:

```rust
let channel = ChannelBuilder::new(Arc::clone(&env))
        .max_send_message_len(100 << 20)
        .max_receive_message_len(100 << 20)
        .set_credentials(creds)  // note the new, dedicated function
        .connect(&endpoint);
```

See the various [examples](googleapis-raw/examples/) for more details.
