FROM rust:1.85.0 AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /src
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
FROM public.ecr.aws/lambda/provided:al2023
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/example /var/runtime/bootstrap
ENTRYPOINT ["/var/runtime/bootstrap"]
