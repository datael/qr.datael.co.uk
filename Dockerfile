FROM rust:latest as builder

RUN cargo install trunk

WORKDIR /src/app
COPY ./Cargo.toml .
COPY ./rust-toolchain.toml .
COPY ./Cargo.lock .
COPY ./src .
COPY ./index.html .

EXPOSE 8080

RUN trunk build --release

CMD ["trunk", "serve", "--release", "--open"]
