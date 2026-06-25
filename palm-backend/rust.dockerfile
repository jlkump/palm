FROM rust:slim as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/palm-backend .

CMD ["./palm-backend"]
