FROM rust:latest as builder

RUN cargo install trunk

WORKDIR /src/app
COPY . .

EXPOSE 8080

RUN trunk build --release

CMD ["trunk", "serve", "--release"]
