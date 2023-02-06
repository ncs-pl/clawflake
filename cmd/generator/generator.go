package main

import (
	"context"

	generatorpb "go.nc0.fr/clawflake/api/nc0/clawflake/generator/v3"
)

// GeneratorServiceServer is an implementation of the gRPC GeneratorService
// service.
type GeneratorServiceServer struct {
	generatorpb.UnimplementedGeneratorServiceServer
}

// Generate allows generating a set of Clawflake ID numbers.
func (g *GeneratorServiceServer) Generate(context.Context, *generatorpb.GenerateRequest) (*generatorpb.GenerateResponse, error) {
	return nil, nil
}
