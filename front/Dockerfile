FROM rust:1.59

RUN apt-get update -y && apt-get upgrade -y

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk