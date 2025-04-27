pub mod sig_to_sk;


use crate::prelude::*;
use crate::prelude::eth::*;



#[derive(Clone)]
pub struct Wallet {
    pub signer: Arc<Box<dyn Signer + Sync>>,
}

#[derive(Debug, Clone)]
pub enum WalletSource {
    Mnemonic(String),
    #[cfg(feature = "trezor")]
    Trezor,
}

impl WalletSource {
    pub fn new_seed() -> AResult<String> {
        let mnemonic: Mnemonic<English> = Mnemonic::new_with_count(&mut OsRng, 24)?;
        Ok(mnemonic.to_phrase())
    }

    pub fn new_mnemonic() -> AResult<Self> {
        let res = Self::Mnemonic(Self::new_seed()?);
        Ok(res)
    }

    pub fn check_backup_phrase(&self, phrase: &str) -> bool {
        #[allow(irrefutable_let_patterns)]
        if let Self::Mnemonic(mnemonic_phrase) = self {
            mnemonic_phrase == phrase
        } else { false }
    }
}

impl Wallet {
    pub async fn new(wallet_source: &WalletSource) -> AResult<Self> {
        let signer: Arc<Box<dyn Signer + Sync>> = match wallet_source {
            WalletSource::Mnemonic(mnemonic_phrase) => {
                Arc::new(Box::new(
                        MnemonicBuilder::<English>::default()
                        .phrase(mnemonic_phrase)
                        .build()?
                ))
            },
            #[cfg(feature = "trezor")]
            WalletSource::Trezor => {
                Arc::new(Box::new(
                        TrezorSigner::new(TrezorLive(0), Some(1))
                        .await?
                ))
            },
        };

        Ok(Wallet { signer })
    }

    pub fn normal_eth_address(&self) -> String {
        self.default_eth_address().to_checksum(Some(1)).to_string()
    }

    pub fn default_eth_address(&self) -> Address {
        self.signer.address()
    }
}

pub enum CreateWalletState {
    ChooseSeedSource,
    ProcessSeedSource,
    CreateWallet(SeedSource),
    VerifyBackup {
        seed: String,
        wallet: Wallet,
    },
    // VerifyBackup,
    Complete(Wallet),
}


impl CreateWalletState {
    pub fn start() -> Self {
        CreateWalletState::ChooseSeedSource
    }

    pub async fn process(
        &mut self,
        input: &str,
        output: &mut String,
    ) -> AResult<(bool, bool)> {
        let mut create_complete = false;

        let mut require_new_input = true;
        match self {
            Self::ChooseSeedSource => {
                *output = SeedSource::generate_seed_source_prompt();
                *self = Self::ProcessSeedSource;
            },
            Self::ProcessSeedSource => {
                let seed_source_idx = input.parse::<usize>()?;
                // info!("happened");
                let seed_source = SeedSource::SEED_SOURCE_OPTIONS[seed_source_idx].clone();
                (*output, require_new_input) = seed_source.hint();
                // (*self, create_complete, require_new_input) = seed_source.process(input, output).await?;
                *self = Self::CreateWallet(seed_source);
            },
            Self::CreateWallet(seed_source) => {
                (*self, create_complete, require_new_input) = seed_source.process(input, output).await?;
            },
            Self::VerifyBackup {
                seed,
                wallet,
            } => {
                if input == seed {
                    *self = Self::Complete(wallet.clone());
                    create_complete = true;
                } else {
                    *output = "The mnemonic words entered are inconsistent with the ones you previously backed up.\nTry again".to_string()
                };
            },
            Self::Complete(_) => {
                // *output = "KeyPair Create Complete".to_string();
                create_complete = true;
            },
        };

        Ok((create_complete, require_new_input))
    }
}

#[cfg(feature = "trezor")]
static PLEASE_OPERATE_HARDWARE_KEY: &str = "Please operate your hardware key.";
#[derive(Clone, strum_macros::Display)]
pub enum SeedSource {
    Random,
    Mnemonic,
    #[cfg(feature = "trezor")]
    Trezor,
}

impl SeedSource {
#[cfg(not(feature = "trezor"))]
    pub const SEED_SOURCE_OPTIONS: [SeedSource; 2] = [
        SeedSource::Random,
        SeedSource::Mnemonic,
    ];
#[cfg(feature = "trezor")]
    pub const SEED_SOURCE_OPTIONS: [SeedSource; 3] = [
        SeedSource::Random,
        SeedSource::Mnemonic,
        SeedSource::Trezor,
    ];

    pub fn generate_seed_source_prompt() -> String {
        let mut prompt = String::from("Please select an option，Enter num：\n");
        for (index, seed_source) in Self::SEED_SOURCE_OPTIONS.iter().enumerate() {
            prompt.push_str(&format!("({}) {}\n", index, seed_source));
        }
        prompt
    }


    pub async fn process(
        &mut self,
        input: &str,
        output: &mut String,
    ) -> AResult<(CreateWalletState, bool, bool)> {
        let mut create_complete = false;

        #[allow(unused_mut)]
        let mut require_new_input = true;
        let res = match self {
            Self::Random => {
                let seed = WalletSource::new_seed()?;
                *output = format!("Please Write Down \"{seed}\"");
                let wallet_source = WalletSource::Mnemonic(seed.clone());
                let wallet = Wallet::new(&wallet_source).await?;
                CreateWalletState::VerifyBackup{
                    seed,
                    wallet,
                }
            },
            Self::Mnemonic => {
                let wallet_source = WalletSource::Mnemonic(input.into());
                let res = Wallet::new(&wallet_source).await;
                match res {
                    Ok(wallet) => {
                        create_complete = true;
                        CreateWalletState::Complete(wallet)
                    },
                    Err(e) => {
                        if input != "1" { *output = e.to_string(); }
                        CreateWalletState::CreateWallet(self.clone())
                    }
                }
            },
            #[cfg(feature = "trezor")]
            Self::Trezor => {
                let wallet_source = WalletSource::Trezor;
                let wallet = Wallet::new(&wallet_source).await?;
                require_new_input = false;
                create_complete = true;
                CreateWalletState::Complete(wallet)
            },
        };

        Ok((res, create_complete, require_new_input))
    }



    pub fn hint(&self) -> (String, bool) {
        let mut require_new_input = false;
        let res = match self {
            Self::Random => "Generating New Keypair",
            Self::Mnemonic => {
                require_new_input = true;
                "Please enter 24 words"
            },
            #[cfg(feature = "trezor")]
            Self::Trezor => PLEASE_OPERATE_HARDWARE_KEY,
        };

        (res.to_string(), require_new_input)
    }
}
