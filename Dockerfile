# -- Stage 1: Build --

FROM rust:1.87.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# -- Stage 2: Runtime -- 

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libpq5 && rm -rf /var/lib/apt/lists/*

RUN useradd -m wow
USER wow

WORKDIR /app

COPY --from=builder /app/target/release/wow-be .

COPY .env .

EXPOSE 3000

CMD ["./wow-be"]
