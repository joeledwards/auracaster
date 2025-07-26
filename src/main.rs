use bluer::{Session, Result};
use bluer::adv::{Advertisement, AdvertisementHandle};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    let session = Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter found");
    let adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;

    println!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);

    let le_advertisement = Advertisement {
        local_name: Some("auracaster".to_string()),
        ..Default::default()
    };

    let handle = adapter.advertise(le_advertisement).await?;

    println!("Advertising started. Press ctrl-c to stop.");
    tokio::signal::ctrl_c().await?;

    println!("Stopping advertising");
    drop(handle);
    time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
