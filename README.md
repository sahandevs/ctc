# CTC - Compile-Time-Config

![Crates.io](https://img.shields.io/crates/v/ctc)

load config files on compile time.

## Usage

- 1: add dependencies

```toml
[dependencies]
ctc = "0.2"
```

- 2: import config files like this:

```rust
ctc::import_conf!("Cargo.toml", cargo);

fn main() {
    println!("package.name: {}", cargo::package::name);
    println!("package.version: {}", cargo::package::version);
    println!("package.edition: {}", cargo::package::edition);
}
```

See the [example project](./example).
