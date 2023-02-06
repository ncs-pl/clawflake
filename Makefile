clean:
	rm -rf bin

GENERATOR_DEPS:=$(shell find cmd/generator -type f -name '*.go')

generator: $(GENERATOR_DEPS)
	go build -o=bin/generator -mod=vendor -race $(GENERATOR_DEPS)

TESTCLIENT_DEPS:=$(shell find cmd/testclient -type f -name '*.go')

testclient: $(TESTCLIENT_DEPS)
	go build -o=bin/testclient -mod=vendor -race $(TESTCLIENT_DEPS)

API_DEPS:=$(shell find api -type f -name '*.proto')

api: $(API_DEPS)
	protoc --proto_path=./ \
  	--proto_path=third_party/googleapis \
  	--proto_path=third_party/grpc-proto \
  	--go_out=. \
  	--go_opt=paths=source_relative \
  	--go-grpc_out=. \
  	--go-grpc_opt=paths=source_relative \
		$(API_DEPS)
