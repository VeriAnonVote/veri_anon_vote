use verifier_core::upsert_voter;
use common::prelude::*;


#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let voter_key = bs58::decode(args.get(1).unwrap())
        .into_vec().unwrap()
        .as_slice().try_into().unwrap();
    let res = upsert_voter(voter_key).await;
    println!("\n'{res:#?}'\n");
}
