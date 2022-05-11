FROM rust:1.60 as builder

WORKDIR /usr/src

COPY . .

RUN rm -rf build.rs

RUN cargo build --release

FROM debian:latest
WORKDIR /opt

RUN mkdir logs

COPY ./env ./env
COPY ./dockerfiles/entrypoint.sh .

COPY --from=builder /usr/src/target/release/customer_services .
COPY --from=builder /usr/src/target/release/product_services .
COPY --from=builder /usr/src/target/release/order_services .
COPY --from=builder /usr/src/target/release/web_api_gateway .

EXPOSE 3030 

# RUN chmod +x /opt/entrypoint.sh

# RUN .entrypoint.sh

ENTRYPOINT ["/bin/bash"]
