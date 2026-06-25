FROM rust:slim as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:latest

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/palm-backend .

CMD ["./palm-backend"]
