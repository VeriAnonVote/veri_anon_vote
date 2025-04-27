use crate::prelude::*;

use std::fs;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub ports: Vec<String>,
    pub sockets: Vec<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
// enum VoterProof {
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterRequirements {
    pub required_identity: String,
    pub allowed_proof_type: Vec<String>,
}

// pub type VoterRegistrationOpen = Arc<AtomicBool>;
pub type RegistrarConfig = Arc<Config>;
#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct Config {
    pub voter_requirements: VoterRequirements,
    pub admin_key: String,
    pub sqlite3_file_path: String,

    pub admin: ServerConfig,
    pub verifier: ServerConfig,
    pub public: ServerConfig,
}



impl Config {
    pub fn from_path(path: &str) -> AResult<RegistrarConfig> {
        let config_str = fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;
            // .map_err(msg)?;

        Ok(Arc::new(config))
    }
}
