fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("gen")
        .compile(
            &[
                "proto/types.proto",
                "proto/customer.proto",
                "proto/product.proto",
                "proto/auth.proto",
                "proto/order.proto",
            ],
            &["proto"],
        )
        .unwrap();

    Ok(())
}
