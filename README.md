# variants

[![license](https://img.shields.io/crates/l/variants.svg?style=for-the-badge&logo=github)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/variants.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/variants)
[![docs.rs](https://img.shields.io/docsrs/variants?style=for-the-badge&logo=docs.rs)](https://docs.rs/variants)

***

The `variants` crates provides a suite of abstractions, utilities, and prodedural macros to facilitate the creation and management of enums and their variants in Rust.

## Features

- [x] `VariantConstructor` - A derive macro for generating functional constructors for enum variants

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/variants.git
cd variants
```

#### _Building the project_

```bash
cargo build --all-features -r -v --workspace
```

#### _Running tests_

```bash
cargo test --all-features -r -v --workspace
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.variants]
features = []
version = "0.1.0"
```

### Examples

#### _Basic Usage_

```rust
    extern crate variants;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to {name}", name = variants);


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
