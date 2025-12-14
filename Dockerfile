
FROM rust:latest as builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/booking ./server

EXPOSE 8080

CMD ["./server"]