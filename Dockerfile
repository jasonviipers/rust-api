FROM rustlang/rust:nightly-slim
ENV ROCKET_ADDRESS=0.0.0.0
RUN apt-get update
RUN apt-get install -y libpq-dev
WORKDIR /usr/src/pip
RUN rustup default nightly;
RUN cargo install diesel_cli --no-default-features --features "postgres";