fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(_) = tonic_build::compile_protos("./proto/auth.proto") {
        Ok(())
    } else {
        panic!()
    }
}
