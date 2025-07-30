# -- Stage 1: Build --

FROM rust:1.87.0 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

RUN cargo build --release

# -- Stage 2: Runtime -- 

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd -m wow
USER wow

WORKDIR /app

COPY --from=builder /app/target/release/wow-be .

EXPOSE 3000

CMD ["./wow-be"]
