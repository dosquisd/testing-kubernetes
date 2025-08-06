FROM rust:1.88.0-slim-bookworm AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock /app/
COPY src /app/src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build /app/target/release/v2 /app/server
CMD [ "/app/server" ]
