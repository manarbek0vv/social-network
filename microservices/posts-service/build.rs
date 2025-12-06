fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../../proto".to_owned();

    tonic_prost_build::configure().compile_protos(
        &[
            format!("{proto_dir}/auth.proto"),
            format!("{proto_dir}/users.proto"),
            format!("{proto_dir}/posts.proto"),
        ],
        &[proto_dir],
    )?;
    Ok(())
}
