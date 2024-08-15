fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hi from build.rs");

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["../proto/helloworld.proto"], &["../proto"])?;

    Ok(())
}
