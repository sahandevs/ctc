mod cargo {
    ctc_utils::import_conf!("cargo");
}

fn main() {
    println!("package.name: {}", cargo::package::name);
    println!("package.version: {}", cargo::package::version);
    println!("package.edition: {}", cargo::package::edition);
}
