FROM rust:1.60 as builder

WORKDIR /usr/src

COPY . .

RUN rm -rf build.rs

RUN cargo build --release

FROM debian:latest
WORKDIR /opt

RUN mkdir logs

#COPY ./env ./env
COPY ./entrypoint.sh .

COPY --from=builder /usr/src/customer_services/target/release/customer_services .
COPY --from=builder /usr/src/product_services/target/release/product_services .
COPY --from=builder /usr/src/web_api_gateway/target/release/web_api_gateway .

EXPOSE 3030 

# RUN chmod +x /opt/entrypoint.sh

# RUN .entrypoint.sh

ENTRYPOINT ["/bin/bash"]
