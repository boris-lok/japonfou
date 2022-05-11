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

pub mod customer_pb {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "gen",
        "/",
        "grpc.customer.rs"
    ));
}

pub mod product_pb {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "gen",
        "/",
        "grpc.product.rs"
    ));
}

pub mod order_item_pb {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "gen",
        "/",
        "grpc.order.rs"
    ));
}
