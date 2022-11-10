# Dependencies
I wasn't really keeping track of all the dependencies; however, I do know that you need to install protoc: https://github.com/protocolbuffers/protobuf


General documentation for Protocol Buffers here: https://developers.google.com/protocol-buffers/docs/proto3

Documentation for gRPC: https://grpc.io/docs/languages/python/quickstart/

Python should yell at you and tell you what you need to pip install:
```
pip install grpcio
pip install grpcio-tools
```

# Running the peasants code
## Server-Python
From the src directory:
```
python3 dataserver.py
```
## Client-Python
From the src directory:
```
python3 dataclient.py
```
## Client-Rust
From the root of data-grpc:
```
cargo run --bin dataclient --release
```