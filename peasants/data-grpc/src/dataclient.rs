use data::data_client::DataClient;
use data::StockDataRequest;

pub mod data {
    tonic::include_proto!("data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DataClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(StockDataRequest {
        ticker: "Tonic".into(), start_date: "tbd".into(), end_date: "tbd".into(),
    });

    let response = client.get_stock_data(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
