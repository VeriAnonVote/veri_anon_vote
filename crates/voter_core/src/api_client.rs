use crate::prelude::*;



// Base URLs stored in the client struct
#[derive(Clone)]
pub struct VoterApiClient {
    pub public_base_url: Url,
    pub client: RequestClient,
    pub wallet: Wallet,
    pub ring_sk: Scalar,
    pub pub_ring: Option<PubRing>,
    pub vote_requirements: Option<VoteRequirements>,
}


#[derive(strum_macros::Display, Debug, Clone, PartialEq)]
pub enum ApiEndpoint { // Add lifetime if params are slices
    #[strum(to_string = "pub_ring")]
    GetPubRing,
    #[strum(to_string = "vote_requirements")]
    GetVoteRequirements,
    // GetAllVoters,
    #[strum(to_string = "vote_record/{record_id}")]
    GetOneVoteRecord { record_id: i32}, // Store data directly
    #[strum(to_string = "vote_record")]
    InsertVoteRecord,
}


impl VoterApiClient{
    // Pass the enum variant directly
    pub fn get_url(&self, endpoint: ApiEndpoint) -> Url {
        let mut url = self.public_base_url.clone();
        let path = &endpoint.to_string();
        url.set_path(path);

        url
    }


    pub async fn new(
        public_base_url: &str,
        client: RequestClient,
        wallet_source: WalletSource,
    ) -> AResult<Self>{
        let wallet = Wallet::new(&wallet_source).await?;
        let mut this = Self {
            public_base_url: Url::parse(public_base_url)?,
            client,
            wallet,
            pub_ring: None,
            ring_sk: Scalar::from_bytes_mod_order([0u8; 32]),
            vote_requirements: None,
        };
        let vote_requirements = this.get_vote_requirements().await?;
        this.ring_sk = this.gen_sk(&vote_requirements.election_name).await?;
        this.vote_requirements = Some(vote_requirements);

        Ok(this)
    }




    pub async fn get_pub_ring(&self) -> AResult<PubRing> {
        // let url = "http://localhost:38080/voter_requirements";
        let url = self.get_url(ApiEndpoint::GetPubRing);
        let res = self.client.get(url)
            .send()
            .await
            .map_err(msg)?
            .json::<PubRing>().await?;

        Ok(res)
    }


    pub async fn vote(
        &self,
        vote_choice: &str,
    ) -> AResult<i32> {
        let pub_ring = self.get_pub_ring().await?;
        let new_vote = NewVoteRecord::new(vote_choice)
            .sign(self.ring_sk, pub_ring);

        let record_id = self.insert_vote_record(&new_vote).await?;
        Ok(record_id)
    }

    pub async fn insert_vote_record(
        &self,
        data: &NewVoteRecord
    ) -> AResult<i32> {
        // let url = "http://localhost:28080/verifier/voter";
        let url = self.get_url(ApiEndpoint::InsertVoteRecord);

        // info!("{data:#?}");
        let res = self.client.post(url)
            .json(data)
            .take_data::<i32>()
            .await;
            // .send()
            // .await
            // .map_err(msg);
        // info!("{res:#?}");

        let record_id = res?;
        Ok(record_id)
    }

    pub async fn get_record_detail(&self, record_id: i32) -> AResult<VoteRecord> {
        let url = self.get_url(ApiEndpoint::GetOneVoteRecord { record_id });
        let res = self.client.get(url)
            .take_data::<VoteRecord>()
            .await?;

        Ok(res)
    }

    pub async fn get_vote_requirements(&self) -> AResult<VoteRequirements> {
        let url = self.get_url(ApiEndpoint::GetVoteRequirements);
        // info!("{url:#?}");
        let res = self.client.get(url)
            .take_data::<VoteRequirements>()
            .await?;

        Ok(res)
    }


    async fn gen_sk(&self, election_name: &str) -> AResult<Scalar> {
        let subkey_purpose_descriptor = format!("This signature generates a subkey for the {election_name}, requiring a secure device.\nThe signature must not be shared with others.");
        let ring_sk_generator = SecretKeyGeneratorBuilder::default()
            .wallet_adderss(self.wallet.normal_eth_address())
            .msg(subkey_purpose_descriptor)
            .build()?;

        ring_sk_generator.sign(&self.wallet)
            .await?
            .generate_scalar()
    }
}
