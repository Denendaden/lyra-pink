FROM rust:1.89 AS builder
WORKDIR /usr/src/lyra-pink
COPY . .
RUN cargo install --path .

FROM ubuntu:latest
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/lyra-pink /usr/local/bin/lyra-pink
ADD www /www
EXPOSE 5566
CMD ["lyra-pink"]

