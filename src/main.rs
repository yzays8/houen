#![deny(unsafe_code)]

use anyhow::Ok;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match houen::run().await {
        Err(houen::Error::ResultNotFound(msg)) => {
            println!("{}", msg);
            Ok(())
        }
        Err(e) => {
            return Err(anyhow::Error::from(e));
        }
        _ => Ok(()),
    }
}
