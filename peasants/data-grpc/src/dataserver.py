import asyncio
import logging
from telnetlib import STATUS
import grpc
import data_pb2
import data_pb2_grpc

class Data(data_pb2_grpc.DataServicer):

    async def getStockData(self, request: data_pb2.StockDataRequest, 
    context :grpc.aio.ServicerContext) -> data_pb2.StockDataResponse:
        logging.info("Message received!")
        stock = data_pb2.StockDataResponse()
        stock.ticker = request.ticker
        stock.start_date = request.start_date
        stock.end_date = request.end_date
        stock.status = data_pb2.StockDataResponse.SUCCESS
        data = stock.data.add()
        data.date = 'tbd'
        data.value = 1.0
        data = stock.data.add()
        data.date = 'tbd'
        data.value = 2.0
        data = stock.data.add()
        data.date = 'tbd'
        data.value = 3.0
        return stock

async def server() -> None:
    data_server = grpc.aio.server()
    data_pb2_grpc.add_DataServicer_to_server(Data(), data_server)
    listen_addr = '[::]:50051'
    data_server.add_insecure_port(listen_addr)
    logging.info("Starting server on %s", listen_addr)
    await data_server.start()
    await data_server.wait_for_termination()

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    asyncio.run(server())