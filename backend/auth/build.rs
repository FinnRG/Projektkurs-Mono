fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(_) = tonic_build::configure()
        .format(false)
        .compile(&["proto/auth.proto"], &["proto/"])
    {
        Ok(())
    } else {
        panic!()
    }
}
