FROM rust:alpine3.21 AS builder 

WORKDIR /usr/src/app 

RUN apk add --no-cache musl-dev

COPY Cargo.toml ./
COPY src ./src 

RUN cargo build --release 

FROM debian:bookworm-slim 

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/backend /app/backend 

EXPOSE 8000

CMD ["/app/backend"]