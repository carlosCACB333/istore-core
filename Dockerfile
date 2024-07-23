FROM rust:1-buster as base
WORKDIR /app

FROM base as development
ENV STAGE=development
WORKDIR /app
RUN cargo install cargo-watch
RUN cargo install diesel_cli
COPY . .
CMD ["cargo", "watch", "-x", "run"]

FROM base as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:12.2 as production
RUN apt-get update && apt-get install -y libpq5
ENV STAGE=production
WORKDIR /app
COPY --from=builder /app/target/release/istore .
CMD ["./istore"]

