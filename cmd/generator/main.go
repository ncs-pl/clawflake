// Package main implements the generator service, a server that can creates
// Clawflake ID numbers.
package main

// TODO(nc0): Configure CI inside //.github/workflows.

import (
	"flag"

	"go.uber.org/zap"
)

var (
	grpcHost *string = flag.String("grpc_host", "localhost:5000", "The host the gRPC server should listen to. Default to localhost:5000.")
	devMode  *bool   = flag.Bool("dev", false, "Enables development mode, with more debug logs.")
)

func main() {
	flag.Parse()

	var logger *zap.Logger
	if *devMode {
		logger, _ = zap.NewDevelopment()
	} else {
		logger, _ = zap.NewProduction()
	}
	defer logger.Sync()

	l := logger.Named("generator").With(zap.Bool("dev", *devMode),
		zap.String("grpc_host", *grpcHost), zap.Uint("epoch", *epoch),
		zap.Uint("machine_id", *machineId))

	// Verify 0 <= machineId < 128
	if *machineId > 128 {
		l.Fatal("Invalid machine_id given, the value should be between 0 and 127 (included).")
		return
	}

	i := NewIdGenerator(l)
	l.Info("i time", zap.Int64("time", i.GetTime()))

	id, err := i.NextId()
	if err != nil {
		l.Error("failed to generate id", zap.Error(err))
		return
	}
	l.Info("i id", zap.Uint64("id", id))

}
