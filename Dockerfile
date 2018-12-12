FROM rust:1.31-stretch AS build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:stretch-slim
RUN apt-get update && apt-get install -y openssl ca-certificates
COPY --from=build /build/target/release/yozhik /bin/
WORKDIR /app
VOLUME /app

CMD yozhik
