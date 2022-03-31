FROM rust:1.59

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features "postgres"
RUN cargo install cargo-watch