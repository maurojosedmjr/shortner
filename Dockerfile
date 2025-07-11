FROM rust:1.88-alpine3.20 as builder
ENV NAME=shortener
WORKDIR /usr/src
WORKDIR /usr/src/${NAME}
RUN apk update && apk add --no-cache musl-dev openssl-dev
COPY . .
ENV RUSTUP_DOWNLOAD_TIMEOUT=180
RUN cargo build --release

FROM rust:1.88-alpine3.20
ENV NAME=shortener
COPY --from=builder /usr/src/${NAME}/target/release/${NAME} /usr/local/bin/${NAME}
EXPOSE 8080
CMD ${NAME}