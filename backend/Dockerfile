FROM rust:latest

WORKDIR /usr/src/chipereganyu_api

COPY .env .env

ENV DB_HOST=${DB_HOST} \
    DB_PORT=${DB_PORT} \
    DB_USER=${DB_USER} \
    DB_PASSWORD=${DB_PASSWORD} \
    DB_NAME=${DB_NAME}

COPY . .

RUN cargo build --release

CMD ["/usr/src/chipereganyu_api"]