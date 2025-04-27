use crate::prelude::*;

use std::fs;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub ports: Vec<String>,
    pub sockets: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRequirements {
    pub election_name: String,
    pub required_identity: String,
    // pub allowed_proof_type: Vec<String>,
}

pub type OrganizerConfig = Arc<Config>;
#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct Config {
    pub vote_requirements: VoteRequirements,
    pub admin_key: String,
    pub sqlite3_file_path: String,
    pub voter_registrar_url: String,

    pub admin: ServerConfig,
    pub public: ServerConfig,
}



impl Config {
    pub fn from_path(path: &str) -> AResult<OrganizerConfig> {
        let config_str = fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;

        Ok(Arc::new(config))
    }
}
