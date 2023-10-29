FROM rust:1.73-bullseye as   builder
WORKDIR /usr/src/chipereganyu_api

COPY .env .env

ENV DB_HOST=${DB_HOST} \
    DB_PORT=${DB_PORT} \
    DB_USER=${DB_USER} \
    DB_PASSWORD=${DB_PASSWORD} \
    DB_NAME=${DB_NAME}


COPY . .


RUN cargo build --release


FROM debian:buster-slim

WORKDIR /usr/src/chipereganyu_api

COPY --from=builder /usr/src/chipereganyu_api/target/release/chipereganyu_api .

EXPOSE 8777

ENTRYPOINT ["chipereganyu_api"]
