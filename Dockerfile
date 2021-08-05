FROM rust:alpine3.14 AS build
RUN cargo install cargo-build-deps \
  && cd /tmp \ 
  && USER=root cargo new --bin clawflake
WORKDIR /tmp/clawflake
COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release
COPY src /tmp/clawflake/src
RUN cargo build --bin clawflake-server --release

FROM alpine:3.14
WORKDIR /app
COPY --from=build /app/target/release/clawflake-server ./
CMD [ "./clawflake-server" ]
