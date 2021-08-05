use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/clawflake.proto")?;

    tonic_build::configure()
        .out_dir(PathBuf::from("./"))
        .build_server(false)
        .compile(&["proto/clawflake.proto"], &["proto"])
        .unwrap();
    Ok(())
}
