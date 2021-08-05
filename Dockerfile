FROM rust:1alpine3.14 AS build
WORKDIR /tmp/clawflake
COPY . ./
RUN cargo build --bin clawflake-server --release

FROM alpine:3.14
WORKDIR /app
COPY --from=build /app/target/release/clawflake-server ./
CMD [ "./clawflake-server" ]
