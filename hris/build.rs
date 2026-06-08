fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/hris.proto", "../platform/proto/platform.proto"],
            &["proto", "../platform/proto"]
        )?;
    Ok(())
}
