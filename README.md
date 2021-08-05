# Clawflake

Clawflake is a Rust application which implements [Twitter Snowflakes](https://github.com/twitter-archive/snowflake/tree/snowflake-2010) and communicates using [gRPC](https://grpc.io/).

Snowflake ID numbers are 63 bits integers stored as `i64`.

An ID number is composed of:

- a timestamp (which is the difference between the current time and the epoch) [`41 bits`]
- a configured machine ID (Data center ID and Worker ID) [`10 bits`]
- a sequence number which rolls over every 4096 per machine (with protection to avoid rollover in the same ms) [`12 bits`]

## Usage

> You need to use the nightly toolchain!

Build the container

```sh
docker build --tag clawflake:1.0 .
```

Run the container

```sh
docker run -e CLAWFLAKE_EPOCH=<epoch> -e CLAWFLAKE_WORKER_ID=<worker_id> -e CLAWFLAKE_DATACENTER_ID=<datacenter_id> -p <host port>:50051 clawflake:1.0
```

You can then create your client using [clawflake.rs](clawflake.rs) and start communicate with the service.

An example client can be found [here](src/client.rs)!
