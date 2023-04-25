package main

import (
	"context"
	"flag"
	"fmt"

	generatorpb "go.nc0.fr/clawflake/api/nc0/clawflake/generator/v3"

	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
	serverAddr *string = flag.String("server", "localhost:5000", "Server URL")
	amount     *uint   = flag.Uint("amount", 1, "Amount of ID numbers to generate.")
)

func main() {
	flag.Parse()
	logger, _ := zap.NewDevelopment()
	l := logger.Named("testclient").With(zap.String("server_addr", *serverAddr), zap.Uint("amount", *amount))

	conn, err := grpc.Dial(*serverAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		l.Error("failed to connect to server", zap.Error(err))
		return
	}
	defer conn.Close()

	c := generatorpb.NewGeneratorServiceClient(conn)
	l.Debug("created client")

	fmt.Println(*amount)

	ids, err := c.Generate(context.Background(), &generatorpb.GenerateRequest{
		Amount: uint32(*amount),
	})
	if err != nil {
		l.Error("failed to generate ID numbers", zap.Error(err))
		return
	}

	l.Info("generated ID numbers")
	for _, v := range ids.IdNumbers {
		fmt.Println(v)
	}
}
