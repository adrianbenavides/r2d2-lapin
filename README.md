[![docs](https://docs.rs/r2d2-lapin/badge.svg)](https://docs.rs/r2d2-lapin)
[![crates.io-version](https://img.shields.io/crates/v/r2d2-lapin)](https://crates.io/crates/r2d2-lapin)
[![tests](https://github.com/adrianbenavides/r2d2-lapin/workflows/Tests/badge.svg)](https://github.com/adrianbenavides/r2d2-lapin/actions)
[![audit](https://github.com/adrianbenavides/r2d2-lapin/workflows/Audit/badge.svg)](https://github.com/adrianbenavides/r2d2-lapin/actions)
[![crates.io-license](https://img.shields.io/crates/l/r2d2-lapin)](LICENSE)

NOTE: This project has been archived. Consider using the [bb8](https://github.com/adrianbenavides/bb8-lapin) or [mobc](https://github.com/zupzup/mobc-lapin) connection managers.

[Lapin](https://github.com/CleverCloud/lapin) support for the [r2d2](https://github.com/sfackler/r2d2) connection pool.

## Usage
See the documentation of r2d2 for the details on how to use the connection pool.

```rust
use lapin::ConnectionProperties;
use r2d2_lapin::LapinConnectionManager;
use std::thread;

fn main() {
    let manager = LapinConnectionManager::new("amqp://guest:guest@127.0.0.1:5672//", &ConnectionProperties::default());
    let pool = r2d2::Pool::builder()
         .max_size(15)
         .build(manager)
         .unwrap();
    
    for _ in 0..20 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            // use the connection
            // it will be returned to the pool when it falls out of scope.
        });
    }
}
```

## Build-time Requirements
The crate is tested on `ubuntu-latest` against the following rust versions: nightly, beta, stable and 1.45.0.
It is possible that it works with older versions as well but this is not tested.
Please see the details of the r2d2 and lapin crates about their requirements.
