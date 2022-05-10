pub mod config;
pub mod json;
pub mod util;

pub mod types {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "gen",
        "/",
        "grpc.types.rs"
    ));
}

pub mod pb {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "gen",
        "/",
        "grpc.customer.rs"
    ));
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "gen",
        "/",
        "grpc.product.rs"
    ));
}
