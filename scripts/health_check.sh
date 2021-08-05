#!/bin/sh

# requires https://github.com/grpc-ecosystem/grpc-health-probe
grpc_health_probe -addr=\[::1]:50051 -service=clawflake.Clawflake
