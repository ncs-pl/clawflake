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

TODO(nc0): Usage information, with or without containers.

## License

Clawflake is governed by a BSD-style license that can be found in the 
[LICENSE](LICENSE) file.

Older codebase was licensed by the Apache License, Version 2.0, however none of
the old code still exists.
