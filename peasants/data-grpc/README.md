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

## Client-React
Help us all. This was a doozy. Reference this first in order to understand why using HTTP2 is so difficult: https://medium.com/swlh/building-a-realtime-dashboard-with-reactjs-go-grpc-and-envoy-7be155dfabfb

The solution is to use Envoy as a passthrough. Gather yourself Peek...we will be using docker for this. The envoy.yaml at the root of react-data-viewer has everything you need. From root of react-data-viewer:
```
docker build -t grpc-medium-envoy:1.0 .
docker run --network=host grpc-medium-envoy:1.0
```
This will eat up your terminal. To see the process running, open another terminal and run:
```
docker ps
```
To kill the process, run:
```
docker kill <name or id>
```

From root of react-data-viewer:
```
npm install
npm start
```
There is a known bug when running for the first time. For some reason you need to delete a statement in App.js, such as,
```
<div className="content">{'Data: ' + myresponse.getDataList()}</div>
```
click save, and then add the statement back, and finally hit save again to see the messages.