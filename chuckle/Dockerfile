# Using the `rust-musl-builder` as base image, instead of 
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --target x86_64-unknown-linux-musl --bin chuckle

FROM alpine AS runtime
WORKDIR /app
# RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/chuckle .
COPY --from=builder /app/migrations migrations
RUN ls -la
# USER myuser
CMD ["/app/chuckle"]
