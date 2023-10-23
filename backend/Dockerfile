FROM rust:1.73-slim-bullseye


WORKDIR /app

COPY . .

RUN cargo build --release


CMD ["./target/release/api-gateway"]