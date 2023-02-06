# Clawflake

Clawflake is a distributed ID number generation system inspired from Twitter 
[Snowflake](https://github.com/twitter-archive/snowflake/tree/snowflake-2010).

The goal of Clawflake is to be hosted as a distributed system with all workers
being isolated from each others apart from the machine ID.

## Format

Unlike Snowflake, the composition of a Clawflake uses all 64 bits.

- `time` (45 bits): The number of milliseconds passed from a configured epoch.
- `sequence` (12 bits): A sequence number rolling out whenever required.
- `machine` (7 bits): An identifier for the worker.

Therefore, Clawflake ID numbers gives **2^45 - 1 = 1115.7 years of safety**
from the configured epoch.  
Thanks to the sequence number, a worker can handle **2^12 = 4069 generations**
per milliseconds at peak.
The system can accept a maximum of **2^7 = 128 machines** for a given epoch.

> Since Clawflake uses the most significant bit, converting a Clawflake ID from
> `uint64` to `int64` is not safe.

## Usage

Before launching any worker, you need to determine the following information:

- `epoch`: corresponds to the epoch workers will be using to generate IDs.
- `machine`: the identifier for the machine.

Due to the format of a Clawflake, you can only have 128 workers (machine IDs 
between 0 and 127).

You can compile the worker by running `make generator`.
This will generate an executable named `generator` inside the `bin/` directory.

You can then start the worker by running:

```shell
export MACHINE_ID=  # Worker ID, between 0 and 127
export EPOCH=  # Epoch to use in ID generation
./bin/generator -machine_id=$MACHINE_ID -epoch=$EPOCH -grpc_host=":5000"
```

> Use the flag `-help` to view the documentation for the flags.

A worker should be running on port `5000`. You can try generating some 
Clawflake ID numbers using the [Generator API](api/nc0/clawflake/generator/v3).
A test client is available in [`cmd/testclient`](cmd/testclient/main.go).

## License

Clawflake is governed by a BSD-style license that can be found in the 
[LICENSE](LICENSE) file.

Older codebase was licensed by the Apache License, Version 2.0, however none of
the old code still exists.
