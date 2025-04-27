pub mod prelude;
pub mod api_client;

#[cfg(not(target_arch = "wasm32"))]
use prelude::*;



#[cfg(not(target_arch = "wasm32"))]
pub async fn upsert_voter_record() -> AResult<()> {

    let onion_client = OnionClient::try_new()?;

    // let wallet_source = WalletSource::new_mnemonic()?;
    let seed = "endless matter defy race stock hotel trumpet parent network analyst youth cradle effort armed seminar chimney friend symptom brother tumble adapt average eternal zone".to_string();
    let wallet_source = WalletSource::Mnemonic(seed);
    // let wallet_source = WalletSource::Trezor;
    let client = api_client::VoterApiClient::new(
        "http://localhost:38081",
        onion_client,
        wallet_source,
    ).await?;

    // println!("\n'{}'\n", wallet.signer.address().to_checksum(Some(1)));
    let pubkey_hex = hex::encode(client.ring_sk.compute_pubkey().to_bytes());
    // println!("\n'{:#?}'\n", client.ring_sk.compute_pubkey());
    // println!("\n'{}'\n", hex::encode(pubkey_hex));
    println!("\n'{}'\n", pubkey_hex);

    let record_id = client.vote("xinjinpig").await?;

    let res = client.get_record_detail(record_id).await?;

    println!("\n'{res:?}'\n");
    Ok(())
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_upsert_voter_record() {
        let res = upsert_voter_record().await;
        println!("\n'{res:#?}'\n");
        assert!(res.is_ok());
    }
}
