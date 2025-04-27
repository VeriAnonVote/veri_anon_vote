use crate::prelude::*;
use crate::models::voter::NewVoter;


// #[derive(Clone, Debug, Builder, Serialize, Deserialize)]
// pub struct Verifier {
//     pub name: String,
//     pub wallet_address: [u8; 20],
//     pub description: Option<String>,
// }

#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
pub struct Message {
    #[serde(with = "base58")]
    #[builder(setter(into))]
    pub voter_pubkey: Vec<u8>,
    #[builder(setter(into))]
    pub voter_info: Option<String>,
    #[builder(setter(into))]
    pub ts: String,
    #[builder(setter(into))]
    pub qualification: String,
    pub version: i16,
}

// #[derive(Clone, Debug, Builder, Serialize, Deserialize)]
// pub struct Qualification {
// election_context: String,
// voter_proof_type: String,
// }

pub fn qualification (
    required_identity: &str,
    proof_type: &str,
) -> String {
    format!(
        "I hereby certify that the holder of the provided public key has been verified as meeting the eligibility requirements for \"{required_identity}\" through the submission of \"{proof_type}\".",
        // "I hereby certify that the holder of the above public key has been verified to meet the eligibility criteria for {} by providing {}.",
        // required_identity, proof_type
    )
}

impl Message {
    pub fn from_metadata(r: &NewVoter, election_context: &str) -> AResult<Self> {
        let qualification = qualification(election_context, &r.proof_type);
        let dt = LocalDateTimeBuilder::default()
            .utc_timestamp(r.utc_timestamp)
            .offset(r.offset)
            .build()?
            .to_datetime()?
            .to_rfc3339();

        MessageBuilder::default()
            .version(1)
            .voter_pubkey(&*r.voter_pubkey)
            .voter_info(r.voter_info.clone())
            .ts(dt)
            .qualification(qualification)
            .build()
            .map_err(msg)
    }

    pub async fn sign(
        &self,
        wallet: &Wallet,
    ) -> AResult<[u8; 65]> {
        let message = self.to_toml()?.into_bytes();
        let signature = wallet.signer
            .sign_message(&message)
            .await?;

        Ok(signature.into())
    }

    pub fn verify(
        &self,
        signature: &[u8],
        signer_address: &[u8],
    ) -> AResult<bool> {
        let signer_address = Address::from_slice(signer_address);
        let signature = Signature::from_raw(signature)?;

        let recoverd = signature.recover_address_from_msg(self.to_toml()?)?;
        // debug!("\n{}\n{}\n", hex::encode(recoverd), hex::encode(signer_address));
        Ok(recoverd == signer_address)
    }

    pub fn to_toml(&self) -> AResult<String> {
        let res = toml::to_string_pretty(self)?
            .replace("\n", "\n\n");

        Ok(res)
    }
}



// impl From<Voter> for Message {
//     fn from(r: Voter) -> Self {
//     let dt = LocalDateTime::now()
//         .set_utc_timestamp(r.utc_timestamp)
//         .to_datetime()
//         .unwrap()
//         .to_rfc3339();

//     MessageBuilder::default()
//         .version(1)
//         .voter_pubkey(r.voter_pubkey)
//         .voter_info(r.voter_info)
//         .ts(dt)
//         .qualification()
//         .build()
//         .unwrap()
//     }
// }




pub fn test_message() -> AResult<()> {
    let dt = LocalDateTime::now()
        .to_datetime()?
        .to_rfc3339();

    let message = MessageBuilder::default()
        .version(1)
        .voter_pubkey(hex::decode("c835d3eb15b665ba9ca87961fe7d4859c7885c38206874140a369de9152c6eab").unwrap())
        .voter_info(None)
        .ts(dt)
        .qualification("approve".to_string())
        .build()?;
    // .map_err(msg)?;
    // let dt_str = format!("{}", dt);
    // println!("\n'{:#?}'\n", message.to_toml()?);
    println!("\n'{}'\n", message.to_toml()?);
    // println!("\n'{}'\n", dt);
    Ok(())
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verifier_message() {
        test_message().unwrap();
        // assert_eq!(counter.count, 1);
    }
}
