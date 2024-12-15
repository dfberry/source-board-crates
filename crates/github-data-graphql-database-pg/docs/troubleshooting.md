# Troubleshooting

## Can't connect to mongo db

```
thread 'main' panicked at crates/github-data-graphql-database/src/main.rs:67:50:
called `Result::unwrap()` on an `Err` value: Error { kind: ServerSelection { message: "Server selection timeout: No available servers. Topology: { Type: Unknown, Servers: [ { Address: mongo:27017, Type: Unknown, Error: Kind: I/O error: failed to lookup address information: Name or service not known, labels: {} } ] }" }, labels: {}, wire_version: None, source: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```