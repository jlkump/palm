FROM rust:slim as builder

WORKDIR /app

COPY . .

RUN cargo build

FROM debian:latest

WORKDIR /usr/local/bin

COPY --from=builder /app/target/debug/palm-backend .

CMD ["./palm-backend"]