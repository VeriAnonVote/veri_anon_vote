use crate::prelude::*;



// Base URLs stored in the client struct
#[derive(Clone)]
pub struct VerifierApiClient<'a> {
    public_base_url: Url,
    verifier_base_url: Url,
    voter_requirements: Option<VoterRequirements>,
    pub client: &'a ClientWithMiddleware,
    pub wallet: Wallet,
    verifier_id: i32,
}

#[derive(strum_macros::Display, Debug, Clone, PartialEq)]
pub enum ApiEndpoint{ // Add lifetime if params are slices
    #[strum(to_string = "voter_requirements")]
    GetVoterRequirements,
    #[strum(to_string = "voters")]
    GetAllVoters,
    #[strum(to_string = "voter/{voter_id}")]
    GetOneVoter { voter_id: i32},
    #[strum(to_string = "verifier/voter")]
    InsertVoter,
}


impl<'a> VerifierApiClient<'a> {
    pub async fn new(
        public_base_url: &str,
        verifier_base_url: &str,
        client: &'a ClientWithMiddleware,
        wallet_source: WalletSource,
        verifier_id: i32,
    ) -> AResult<Self>{
        let wallet = Wallet::new(&wallet_source).await?;
        let mut this = Self {
            public_base_url: Url::parse(public_base_url)?,
            verifier_base_url: Url::parse(verifier_base_url)?,
            client,
            voter_requirements: None,
            wallet,
            verifier_id
        };

        this.voter_requirements = Some(this.get_voter_requirements().await?);

        Ok(this)
    }



    // Pass the enum variant directly
    pub fn get_url(&self, endpoint: ApiEndpoint) -> Url {
        let base_url = match endpoint {
            ApiEndpoint::InsertVoter => &self.verifier_base_url,
            _ => &self.public_base_url,
        };
        let path = &endpoint.to_string();
        let mut res = base_url.clone();
        res.set_path(path);

        res
    }



    // Example usage in an API method
    pub async fn get_voter_requirements(&self) -> AResult<VoterRequirements> {
        // let url = "http://localhost:38080/voter_requirements";
        let url = self.get_url(ApiEndpoint::GetVoterRequirements);
        let res = self.client.get(url)
            .take_data().await?;

        Ok(res)
    }



    pub async fn insert_voter(
        &self,
        voter_pubkey: [u8; 32],
        proof_type_idx: usize
    ) -> AResult<i32> {
        let url = self.get_url(ApiEndpoint::InsertVoter);
        let voter_requirements = &self.voter_requirements();
        let voter_proof_type = &voter_requirements.allowed_proof_type[proof_type_idx];
        let qualification = qualification(&voter_requirements.required_identity, voter_proof_type);

        let dt = LocalDateTime::now();
        let ts = dt.to_datetime()?
            .to_rfc3339();

        let message: Message = MessageBuilder::default()
            .version(1)
            .voter_pubkey(voter_pubkey)
            .voter_info(None)
            .ts(ts)
            .qualification(qualification)
            .build()?;

        let voter_data = NewVoter::new(
            message,
            dt,
            &self.wallet,
            voter_proof_type,
            self.verifier_id,
        ).await?;

        let voter_id = self.client.post(url)
            .json(&voter_data)
            .take_data()
            .await?;

        // let voter_id = res.json().await?;
        Ok(voter_id)
    }



    pub async fn get_voter_details(&self, voter_id: i32) -> AResult<Voter> {
        // let voter_url = format!("http://localhost:38080/voter/{voter_id}");
        let url = self.get_url(ApiEndpoint::GetOneVoter { voter_id });
        // let url = self.build_public_url(PublicApiEndpoint::GetOneVoter, Some(voter_id))?;
        let res = self.client.get(url)
            .take_data::<Voter>().await?;

        Ok(res)
    }



    pub fn voter_requirements(&self) -> VoterRequirements {
        self.voter_requirements.clone().unwrap()
    }
}
