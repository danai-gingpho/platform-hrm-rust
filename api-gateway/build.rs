fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile_protos(
            &[
                "proto/auth.proto",
                "proto/hris.proto",
                "proto/platform.proto",
                "proto/owner.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
