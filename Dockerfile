# Use an official Rust runtime as a parent image
FROM rust:latest AS builder

WORKDIR /usr/src/interstellar-ai-explorer

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/interstellar-ai-explorer/target/release/interstellar-ai-explorer /usr/local/bin/interstellar-ai-explorer

CMD ["interstellar-ai-explorer"]
