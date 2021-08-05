FROM rustlang/rust:nightly-slim AS build
WORKDIR /app
COPY . /app
RUN cargo build --release --bin clawflake-server

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=build /app/target/release/clawflake-server ./
CMD [ "./clawflake-server" ]
