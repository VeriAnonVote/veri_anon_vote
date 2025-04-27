use crate::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use crate::schema::{
    self,
    vote_record::{
        self,
        dsl,
    },
};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Queryable, Selectable, AsChangeset, Insertable, Clone, Debug, Deserialize, Serialize,)]
#[diesel(table_name = schema::vote_record)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VoteRecord {
    pub id: i32,
    pub vote_choice: String,
    // #[serde(with = "serde_json")]
    pub ring_sig: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, Deserialize, Serialize,)]
pub struct VoteRecord {
    pub id: i32,
    pub vote_choice: String,
    // #[serde(with = "serde_json")]
    pub ring_sig: Vec<u8>,
}


#[cfg(not(target_arch = "wasm32"))]
impl VoteRecord {
    pub fn get_one (
        id: i32,
        pool: &impl ConnectionProvider,
    ) -> AResult<VoteRecord> {
        let vote = dsl::vote_record.find(id)
            .first(&mut pool.conn()?)?;
        Ok(vote)
    }


    pub fn get_all (
        pool: &impl ConnectionProvider,
    ) -> AResult<Vec<VoteRecord>> {
        let records = dsl::vote_record.filter(vote_record::id.gt(0))
            .load(&mut pool.conn()?)?;
        Ok(records)
    }
}


#[cfg(not(target_arch = "wasm32"))]
#[derive(Builder, AsChangeset, Insertable, Clone, Debug, Deserialize, Serialize,)]
#[diesel(table_name = schema::vote_record)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewVoteRecord {
    #[builder(setter(into))]
    pub vote_choice: String,
    // #[serde(with = "serde_json")]
    #[builder(default = "Vec::new()")]
    pub ring_sig: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Builder, Clone, Debug, Deserialize, Serialize,)]
pub struct NewVoteRecord {
    #[builder(setter(into))]
    pub vote_choice: String,
    // #[serde(with = "serde_json")]
    #[builder(default = "Vec::new()")]
    pub ring_sig: Vec<u8>,
}

impl NewVoteRecord {
    pub fn new(vote_choice: &str) -> Self{
        NewVoteRecordBuilder::default()
            .vote_choice(vote_choice)
            .build()
            .unwrap()
    }



#[cfg(not(target_arch = "wasm32"))]
    pub fn check_signature_uniqueness(
        &self,
        all_records: &[VoteRecord],
        pub_ring: &PubRing,
    ) -> bool {
        serde_json::from_slice(&self.ring_sig)
            .is_ok_and(|new_sig: BLSAG| {
                if new_sig.ring != *pub_ring {
                // info!("\n{:?}\n{:?}", new_sig.ring, pub_ring);
                        return false;
                }
                // info!("{:#?}", &new_sig);
                for val in all_records {
                    let sig = serde_json::from_slice(&val.ring_sig)
                        .unwrap();
                    if BLSAG::link(new_sig.clone(), sig) {
                        return false;
                    }
                }
                true
            })
    }

    pub fn verify_signature(&self) -> bool {
        let message = voter_choice_message(&self.vote_choice);
        serde_json::from_slice(&self.ring_sig)
            .is_ok_and(|signature|
                !BLSAG::verify::<Keccak512>(signature, &message)
            )
    }

    pub fn sign(
        mut self,
        sk: Scalar,
        pub_ring: Vec<RistrettoPoint>,
    ) -> Self {
        let message = voter_choice_message(&self.vote_choice);
        let pub_key: RistrettoPoint = sk * RISTRETTO_BASEPOINT_POINT;
        let (new_ring, secret_index) = exclude_voter_key(&pub_ring, pub_key);

        let sig = BLSAG::sign::<Keccak512, OsRng>(sk, new_ring, secret_index, &message);
        self.ring_sig = serde_json::to_vec(&sig).unwrap();

        self
    }
}

fn exclude_voter_key(
    pub_ring: &[RistrettoPoint],
    pub_key: RistrettoPoint,
) -> (Vec<RistrettoPoint>, usize) {
    let mut secret_index = 0;
    let new_ring: Vec<RistrettoPoint> = pub_ring
        .iter()
        .enumerate()
        .filter_map(|(idx, &point)| {
            if point == pub_key {
                secret_index = idx;
                None
            } else {
                Some(point)
            }
        })
        .collect();
    (new_ring, secret_index)
}

pub fn voter_choice_message(choice: &str) -> Vec<u8> {
    format!("I choose {}", choice)
        .as_bytes()
        .to_vec()
}
// pub fn voter_choice_message(
//     id: i16,
//     available_choices: Vec<String>,
//     election_name: String,
// ) -> String {
//     format!("I choose {} in the {election_name}", available_choices[id as usize])
// }
// pub fn voter_choice_message_as_bytes(
//     id: i16,
//     available_choices: Vec<String>,
//     election_name: String,
// ) -> String {
//     voter_choice_message(id, available_choices, election_name).as_bytes()
// }
