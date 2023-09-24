fn main() {
    let mut metadata = cargo_metadata::MetadataCommand::new();
    metadata.manifest_path("Cargo.toml");

    println!("{}", metadata.exec().unwrap().target_directory);
}
