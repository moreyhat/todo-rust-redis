FROM rust:1.67 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc
COPY --from=builder /usr/local/cargo/bin/todo-rust-redis /usr/local/bin/todo-rust-redis

EXPOSE 8080
CMD ["/usr/local/bin/todo-rust-redis"]