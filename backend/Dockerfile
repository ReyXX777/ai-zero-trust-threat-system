FROM rust:1.64

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release
CMD ["./target/release/ai-zero-trust-threat-system"]
