use crate::prelude::*;

use crate::verifier_sig::Message;
#[cfg(not(target_arch = "wasm32"))]
use crate::models::verifier::Verifier;
#[cfg(not(target_arch = "wasm32"))]
use crate::schema::{
    self,
    voter::dsl,
};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Associations, Queryable, Selectable, AsChangeset, Insertable, Clone, Debug, Deserialize, Serialize,)]
#[diesel(table_name = crate::schema::voter)]
#[diesel(belongs_to(Verifier))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Voter {
    pub id: i32,
    pub verifier_id: i32,

    // pub name: Option<String>,
    pub proof_type: String,
    pub utc_timestamp: i64,
    pub offset: i32,
    #[serde(with = "base58")]
    pub voter_pubkey: Vec<u8>,
    pub version: i16,
    #[serde(with = "hex::serde")]
    pub verifier_sig: Vec<u8>,
    pub voter_info: Option<String>,
}


#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, Deserialize, Serialize,)]
pub struct Voter {
    pub id: i32,
    pub verifier_id: i32,

    // pub name: Option<String>,
    pub proof_type: String,
    pub utc_timestamp: i64,
    pub offset: i32,
    #[serde(with = "base58")]
    pub voter_pubkey: Vec<u8>,
    pub version: i16,
    #[serde(with = "hex::serde")]
    pub verifier_sig: Vec<u8>,
    pub voter_info: Option<String>,
}



#[cfg(not(target_arch = "wasm32"))]
impl Voter {
    pub fn get_one (
        id: i32,
        pool: &impl ConnectionProvider,
    ) -> AResult<Voter> {
        let voters = dsl::voter.find(id)
            .first(&mut pool.conn()?)?;
        Ok(voters)
    }


    pub fn get_all (
        pool: &impl ConnectionProvider,
    ) -> AResult<Vec<Voter>> {
        let voters = dsl::voter.filter(schema::voter::id.gt(0))
            .load(&mut pool.conn()?)?;
        Ok(voters)
    }
}



#[cfg(not(target_arch = "wasm32"))]
#[derive(Builder, AsChangeset, Insertable, Clone, Debug, Deserialize, Serialize,)]
#[diesel(table_name = crate::schema::voter)]
#[diesel(belongs_to(Verifier))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewVoter {
    pub verifier_id: i32,
    #[builder(setter(into))]
    pub proof_type: String,
    pub utc_timestamp: i64,
    pub offset: i32,
    #[builder(setter(into))]
    #[serde(with = "base58")]
    pub voter_pubkey: Vec<u8>,
    pub version: i16,
    #[builder(setter(into))]
    #[serde(with = "hex::serde")]
    pub verifier_sig: Vec<u8>,
    pub voter_info: Option<String>,
}


#[cfg(target_arch = "wasm32")]
#[derive(Builder, Clone, Debug, Deserialize, Serialize,)]
pub struct NewVoter {
    pub verifier_id: i32,
    #[builder(setter(into))]
    pub proof_type: String,
    pub utc_timestamp: i64,
    pub offset: i32,
    #[builder(setter(into))]
    #[serde(with = "base58")]
    pub voter_pubkey: Vec<u8>,
    pub version: i16,
    #[builder(setter(into))]
    #[serde(with = "hex::serde")]
    pub verifier_sig: Vec<u8>,
    pub voter_info: Option<String>,
}


impl NewVoter {
    pub async fn new(
        msg: Message,
        dt: LocalDateTime,
        wallet: &Wallet,
        proof_type: &str,
        verifier_id: i32,
    ) -> AResult<NewVoter> {
        // let signature = msg.sign(wallet).await?;
        let res = NewVoterBuilder::default()
            .verifier_id(verifier_id)
            .proof_type(proof_type)
            .utc_timestamp(dt.utc_timestamp)
            .offset(dt.offset)
            .voter_pubkey(&*msg.voter_pubkey)
            .version(msg.version)
            .verifier_sig(msg.sign(wallet).await?)
            .voter_info(msg.voter_info)
            .build()?;

            Ok(res)
    }
}
