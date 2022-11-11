all:
	python3 -m grpc_tools.protoc -I. --python_out=../src --grpc_python_out=../src *.proto
	protoc -I. --js_out=import_style=commonjs,binary:../src/react-data-viewer/src *.proto
	protoc -I. --grpc-web_out=import_style=commonjs,mode=grpcwebtext:../src/react-data-viewer/src *.proto