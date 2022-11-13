from __future__ import print_function
import logging
import grpc
import data_pb2
import data_pb2_grpc

def run():
    print("Trying to run Data Client...")
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = data_pb2_grpc.DataStub(channel)
        response = stub.getStockData(data_pb2.StockDataRequest(ticker='test',
        start_date='11/07/2019', end_date='11/07/2022'))
    print ("Data Client received:")
    print ("ticker =",response.ticker)
    print ("start_date =",response.start_date)
    print ("end_date =",response.end_date)
    print ("status:",(response.DESCRIPTOR.enum_types_by_name['Status']).values_by_number[response.status].name)
    for data in response.data:
        print ("Data \n\tdate:",data.date)
        print ("\n\tvalue:",data.value)

if __name__ == '__main__':
    logging.basicConfig()
    run()