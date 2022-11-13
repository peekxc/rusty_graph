import logo from './logo.svg';
import './App.css';
//const grpc = require("@grpc/grpc-js")

const {StockDataRequest, StockDataResponse} = require('./data_pb.js');
const {DataClient} = require('./data_grpc_web_pb.js');

var dataservice = new DataClient('http://localhost:8000')

var request = new StockDataRequest();
request.setTicker('js');
request.setStartDate('tbd');
request.setEndDate('tbd');

var myresponse = new StockDataResponse()

dataservice.getStockData(request, null, (err, response) => {
  myresponse = response
});


function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" /> 
        <p>
          Edit <code>src/App.js</code> and save to reload.
          <div className="content">{'Ticker: ' + myresponse.getTicker()}</div>
          <div className="content">{'Start Date: ' + myresponse.getStartDate()}</div>
          <div className="content">{'End Date: ' + myresponse.getEndDate()}</div>
          <div className="content">{'Status (3=Success): ' + myresponse.getStatus()}</div>
          <div className="content">{'Data: ' + myresponse.getDataList()}</div>
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
