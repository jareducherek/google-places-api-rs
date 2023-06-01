* `cargo prisma generate`
* `cargo run --example basic_search`

# Repo Structure

    ├── src
    │   ├── lib.rs
    │   ├── error.rs
    │   ├── client.rs
    │   ├── models.rs
    │   ├── services
    │   │   ├── places.rs
    │   │   └── ...
    │   └── utils
    │       ├── request.rs
    │       └── ...
    ├── tests
    │   ├── integration.rs
    │   └── unit
    │       ├── client.rs
    │       ├── services
    │       │   ├── places.rs
    │       │   └── ...
    │       └── ...
    ├── examples
    │   ├── main.rs
    │   ├── basic_search.rs
    │   └── ...
    ├── Cargo.toml
    └── README.md
