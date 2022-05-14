mod bisq {
    tonic::include_proto!("io.bisq.protobuffer");
}

use bisq::get_trades_request::Category;
use bisq::get_version_client::GetVersionClient;
use bisq::trades_client::TradesClient;
use bisq::wallets_client::WalletsClient;
use bisq::{GetTradesRequest, GetVersionRequest, UnlockWalletRequest};
use std::time::Duration;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = std::env::var("BISQ_PASSWORD")?;
    let wallet_password = std::env::var("WALLET_PASSWORD")?;

    let mut client = GetVersionClient::connect("http://localhost:9998").await?;

    let mut req = Request::new(GetVersionRequest {});
    req.set_timeout(Duration::from_secs(5));

    let metadata = req.metadata_mut();
    metadata.insert("password", password.parse()?);

    let response = client.get_version(req).await?.into_inner();
    println!("bisq version: {}", response.version);

    let mut client = WalletsClient::connect("http://localhost:9998").await?;

    let mut req = Request::new(UnlockWalletRequest {
        password: wallet_password,
        timeout: 10,
    });
    req.set_timeout(Duration::from_secs(5));

    let metadata = req.metadata_mut();
    metadata.insert("password", password.parse()?);

    client.unlock_wallet(req).await?.into_inner();

    let mut client = TradesClient::connect("http://localhost:9998").await?;

    let mut req = Request::new(GetTradesRequest {
        category: Category::Closed.into(),
    });
    req.set_timeout(Duration::from_secs(5));

    let metadata = req.metadata_mut();
    metadata.insert("password", password.parse()?);

    let response = client.get_trades(req).await?.into_inner();
    println!("trades: {:?}", response.trades);

    Ok(())
}
