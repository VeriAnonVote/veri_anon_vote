use crate::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use crate::schema::{
    self,
    verifier::dsl,
};

fn max_upload_count() -> i32 {
    10000
}

#[cfg(target_arch = "wasm32")]
#[derive(ToSchema, Clone, Debug, Deserialize, Serialize,)]
pub struct Verifier {
    pub id: i32,
    pub name: String,
    #[serde(with = "eth_address")]
    pub wallet_address: Vec<u8>,
    pub max_upload_count: i32,
    pub api_key: String,
    pub description: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(ToSchema, Queryable, Selectable, AsChangeset, Insertable, Clone, Debug, Deserialize, Serialize,)]
#[diesel(table_name = crate::schema::verifier)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Verifier {
    pub id: i32,
    pub name: String,
    #[serde(with = "eth_address")]
    pub wallet_address: Vec<u8>,
    pub max_upload_count: i32,
    pub api_key: String,
    pub description: Option<String>,
}


#[cfg(not(target_arch = "wasm32"))]
impl Verifier {
    pub fn get_all (
        pool: &impl ConnectionProvider,
    ) -> AResult<Vec<Verifier>> {
        let verifiers = dsl::verifier.filter(schema::verifier::id.gt(0))
            .load(&mut pool.conn()?)?;
        Ok(verifiers)
    }
}



pub type VerifierMap = Arc<DashMap<String, Verifier>>;

pub trait DashMapVerifierExt {
    fn new_arc() -> Self;

    fn find_by_id(
        &self,
        id: i32,
    ) -> Option<Verifier>;

#[cfg(not(target_arch = "wasm32"))]
    fn refresh(
        &self,
        pool: &impl ConnectionProvider,
    ) -> AResult<()>;
}

impl DashMapVerifierExt for VerifierMap {
    fn new_arc() -> Self {
        Arc::new(DashMap::new())
    }

    fn find_by_id(
        &self,
        id: i32,
    ) -> Option<Verifier> {
        self.iter()
            .find(|entry| entry.value().id == id)
            .map(|entry| entry.value().clone())
    }

#[cfg(not(target_arch = "wasm32"))]
    fn refresh(
        &self,
        pool: &impl ConnectionProvider,
    ) -> AResult<()>{
        let verifiers = Verifier::get_all(pool)?;
        // info!("{:#?}", verifiers);
        self.clear();
        for val in verifiers {
            let api_key = val.api_key.clone();
            self.insert(api_key, val);
        }
        // info!("{:#?}", self);

        Ok(())
    }

}



#[cfg(target_arch = "wasm32")]
#[derive(ToSchema, Clone, Debug, Deserialize, Serialize,)]
pub struct NewVerifier {
    pub name: String,
    #[serde(with = "eth_address")]
    pub wallet_address: Vec<u8>,
    #[serde(default = "max_upload_count")]
    pub max_upload_count: i32,
    pub api_key: Option<String>,
    pub description: Option<String>,
}


#[cfg(not(target_arch = "wasm32"))]
#[derive(ToSchema, Insertable, AsChangeset, Clone, Debug, Deserialize, Serialize,)]
#[diesel(table_name = crate::schema::verifier)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewVerifier {
    pub name: String,
    #[serde(with = "eth_address")]
    pub wallet_address: Vec<u8>,
    #[serde(default = "max_upload_count")]
    pub max_upload_count: i32,
    pub api_key: Option<String>,
    pub description: Option<String>,
}


#[cfg(not(target_arch = "wasm32"))]
#[derive(Selectable, Queryable, Serialize)]
#[diesel(table_name = crate::schema::verifier)]
pub struct PublicReqVerifier {
    pub id: i32,
    pub name: String,
    #[serde(with = "eth_address")]
    pub wallet_address: Vec<u8>,
    #[serde(default = "max_upload_count")]
    pub max_upload_count: i32,
    pub description: Option<String>,
}
