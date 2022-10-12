fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
                &["./proto/n1c00o/clawflake/v2/clawflake.proto"],
                &["./proto", "./third_party/googleapis"]
        )
        .unwrap();
    Ok(())
}
