pub mod prelude;
pub mod api_client;

use prelude::*;
use api_client::*;



pub async fn upsert_voter(key: [u8; 32]) -> AResult<()> {
    let onion_client = OnionClient::try_new()?;
    // let seed = "endless matter defy race stock hotel trumpet parent network analyst youth cradle effort armed seminar chimney friend symptom brother tumble adapt average eternal zone".to_string();
    let wallet_source = WalletSource::Trezor;
    let client = VerifierApiClient::new(
        "http://localhost:38080",
        "http://localhost:28080",
        &onion_client,
        wallet_source,
        0,
    ).await?;

    // println!("\n'{:?}'\n", res);
    // println!("\n'{}'\n", client.wallet.normal_eth_address());
    let proof_type_idx = 0usize;

    let voter_id = client.insert_voter(key, proof_type_idx).await?;
    let res = client.get_voter_details(voter_id).await?;

    println!("\n'{res:#?}'\n");
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upsert_voter() {

        let mut keys = [[0u8; 32]; 10];
        for key in keys.iter_mut() {
            OsRng.fill_bytes(key);
            let sk = Scalar::from_bytes_mod_order(*key);
            // RistrettoPoint::from_bytes(key)
            *key = sk.compute_pubkey().to_bytes();
        }

        // keys[6] = hex::decode("40375b886a2df1fc993f7cb99fe49594beecf12c11147877e0ad2d8a551e5e2f").unwrap().try_into().unwrap();
        keys[6] = hex::decode("7c432255f7405f39598e48f3cb75cba4c70d81925fbf1db56504b185456e2713").unwrap().try_into().unwrap();

        for key in keys.iter_mut() {
            let res = upsert_voter(*key).await;
            println!("\n'{res:#?}'\n");
            assert!(res.is_ok());
        }
    }


    #[tokio::test]
    async fn test_urls() {
        let onion_client = OnionClient::default().unwrap();
        let seed = "endless matter defy race stock hotel trumpet parent network analyst youth cradle effort armed seminar chimney friend symptom brother tumble adapt average eternal zone".to_string();
        let wallet_source = WalletSource::Mnemonic(seed);
        let client = VerifierApiClient::new(
            "http://localhost:38080",
            "http://localhost:28080",
            &onion_client,
            wallet_source,
            0,
        ).await.unwrap();

        print!("\n");
        for val in [
            ApiEndpoint::GetVoterRequirements,
            ApiEndpoint::GetAllVoters,
            ApiEndpoint::GetOneVoter { voter_id: "1".to_string()},
            ApiEndpoint::InsertVoter,
        ] {
            println!("{}", client.get_url(val));
        }
    }
}
