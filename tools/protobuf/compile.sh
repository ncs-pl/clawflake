#!/usr/bin/env sh
DEPS=$(find api -type f -name "*.proto")
protoc --proto_path=./ \
  --proto_path=third_party/googleapis \
  --proto_path=third_party/grpc-proto \
  --go_out=. \
  --go_opt=paths=source_relative \
  --go-grpc_out=. \
  --go-grpc_opt=paths=source_relative \
  "$DEPS"