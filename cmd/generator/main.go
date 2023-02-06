// Package main implements the generator service, a server that can creates
// Clawflake ID numbers.
package main

import (
	"flag"
	"net"

	generatorpb "go.nc0.fr/clawflake/api/nc0/clawflake/generator/v3"

	"go.uber.org/zap"
	"google.golang.org/grpc"
)

var (
	grpcHost *string = flag.String("grpc_host", ":5000", "The host the gRPC server should listen to.")
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

	// gRPC server
	lis, err := net.Listen("tcp", *grpcHost)
	if err != nil {
		l.Error("failed to listen", zap.Error(err))
		return
	}
	defer lis.Close()

	l.Info("server is ready")
	gs := grpc.NewServer()
	generatorpb.RegisterGeneratorServiceServer(gs, NewGeneratorServiceServer(i, l))
	if err := gs.Serve(lis); err != nil {
		l.Error("failed to serve gRPC", zap.Error(err))
		return
	}
}
