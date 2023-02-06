package main

import (
	"context"
	"fmt"

	"github.com/google/uuid"
	generatorpb "go.nc0.fr/clawflake/api/nc0/clawflake/generator/v3"
	"go.uber.org/zap"
	"google.golang.org/genproto/googleapis/rpc/errdetails"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// GeneratorServiceServer is an implementation of the gRPC GeneratorService
// service.
type GeneratorServiceServer struct {
	generatorpb.UnimplementedGeneratorServiceServer

	idgen  *IdGenerator
	logger *zap.Logger
}

// Generate allows generating a set of Clawflake ID numbers.
func (g *GeneratorServiceServer) Generate(ctx context.Context, req *generatorpb.GenerateRequest) (*generatorpb.GenerateResponse, error) {
	l := g.logger.With(zap.Uint32("amount", req.Amount), zap.String("request_id", uuid.NewString()))
	l.Info("treating Generate() RPC")

	// Verify request:
	if req.Amount < 1 || req.Amount > 4096 {
		g.logger.Info("rejecting request due invalid amount")
		s := status.New(codes.InvalidArgument, "invalid amount")
		s.WithDetails(&errdetails.BadRequest_FieldViolation{
			Field:       "amount",
			Description: "Amount should be a value between 1 and 4096 (included).",
		})
		return nil, s.Err()
	}

	// Generation:
	ids := make([]string, req.Amount)
	l.Debug("generating ID numbers")
	for i := 0; i < int(req.Amount); i++ {
		id, err := g.idgen.NextId()
		if err != nil {
			l.Error("failed to generate ID", zap.Error(err))
			s := status.New(codes.Internal, "internal error")
			s.WithDetails(&errdetails.ErrorInfo{
				Reason: "INTERNAL",
				Domain: "generator.clawflake.nc0.fr",
			})
			return nil, s.Err()
		}

		l.Debug("generated an ID number", zap.Uint64("generated_id", id))

		ids = append(ids, fmt.Sprint(id))
	}
	l.Debug("generated ID numbers")

	return &generatorpb.GenerateResponse{
		IdNumbers: ids,
	}, nil
}

func NewGeneratorServiceServer(i *IdGenerator, l *zap.Logger) *GeneratorServiceServer {
	return &GeneratorServiceServer{
		idgen:  i,
		logger: l.Named("grpc_generator"),
	}
}
