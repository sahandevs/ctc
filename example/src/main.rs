ctc::import_conf!("example/Cargo.toml", cargo);

fn main() {
    println!("package.name: {}", cargo::package::name);
    println!("package.version: {}", cargo::package::version);
    println!("package.edition: {}", cargo::package::edition);
}
