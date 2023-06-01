* `cargo prisma generate`
# Working Examples:
* `cargo run --example nearby_search`
* `cargo run --example place_details`

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
