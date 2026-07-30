FROM rust:1.91

WORKDIR /app


COPY app ./app
COPY bridge ./bridge

WORKDIR /app/app/bridge_app

RUN cargo build --release

CMD ["./target/release/bridge_app"]