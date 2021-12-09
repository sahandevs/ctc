# CTC - Compile-Time-Config

load config files on compilation.

## Usage

- 1: add dependencies

```toml
[dependencies]
ctc-utils = "0.1"

[build-dependencies]
ctc = "0.1"
```

- 2: create a file named [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
- 3: add the following code:

```rust
use ctc;
fn main() {
    // change the `Cargo.toml` and `cargo` to your desired
    // file path and name.
    ctc::load("Cargo.toml", "cargo").unwrap();
}
```

- 4: finally, use it like this:

```rust
mod cargo {
    ctc_utils::import_conf!("cargo");
}

fn main() {
    println!("package.name: {}", cargo::package::name);
    println!("package.version: {}", cargo::package::version);
    println!("package.edition: {}", cargo::package::edition);
}
```

See the [example](./example) project.
