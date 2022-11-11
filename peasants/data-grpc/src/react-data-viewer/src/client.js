const {StockDataRequest, StockDataResponse} = require('./data_pb.js');
const {DataClient} = require('./data_grpc_web_pb.js');

var dataservice = new DataClient('localhost:50051');

var request = new StockDataRequest();
request.setTicker('js');
request.setStartDate('tbd');
request.setEndDate('tbd');
console.log(request.getTicker());

dataservice.getStockData(request, {}, function(err, response) {
  if (err) {
    console.log(err.code);
    console.log(err.message);
  } else {
    console.log(response.getTicker());
  }
});