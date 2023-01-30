fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/services/proto/user_service.proto")?;
    Ok(())
}
