fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("gen")
        .compile(
            &[
                "proto/customer.proto",
                "proto/product.proto",
                "proto/auth.proto",
            ],
            &["proto"],
        )
        .unwrap();

    Ok(())
}
