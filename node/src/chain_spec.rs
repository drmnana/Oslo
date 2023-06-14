use hex_literal::hex;
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{ Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
// use sp_runtime::key_types::IM_ONLINE;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;

use storage_chain_runtime::{
	currency::*, opaque::SessionKeys, AccountId, Balance, BalancesConfig, EVMConfig,
	GenesisAccount, GenesisConfig, SessionConfig, SudoConfig, SystemConfig,
	WASM_BINARY, GrandpaConfig, AuraConfig , ValidatorSetConfig, ImOnlineConfig, 
};
// use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, default::Default};
// use frame_benchmarking::frame_support::metadata::StorageEntryModifier::Default;
// use libsecp256k1::{PublicKey, PublicKeyFormat};
// use sha3::{Digest};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

// type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
fn get_from_secret<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None)
		.unwrap_or_else(|_| panic!("Invalid string '{}'", seed))
		.public()
}


const ALITH: &str = "0x6B7CD45dfc550F12b4EdAFDFbBC68b53faAE6Fe2";
const BALTATHAR: &str = "0x18119Bb0f49ee709104CA2804B297B08d5d0EDEc";
const CHARLETH: &str = "0x71B18c74b51E2195c92C169504f7FAFA71308A9a";
const DOROTHY: &str = "0xC03cfc225Ad4b42F96f612BA38bD4d9cBD4a419a";

pub fn public_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Public Node",
		"public_live",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						get_from_secret::<AuraId>("//Alice"),
						get_from_secret::<GrandpaId>("//Alice"),
						get_from_secret::<ImOnlineId>("//Alice"),
					),
					(
						array_bytes::hex_n_into_unchecked(BALTATHAR),
						get_from_secret::<AuraId>("//Bob"),
						get_from_secret::<GrandpaId>("//Bob"),
						get_from_secret::<ImOnlineId>("//Bob"),

					),
					(
						array_bytes::hex_n_into_unchecked(CHARLETH),
						get_from_secret::<AuraId>("//Charlie"),
						get_from_secret::<GrandpaId>("//Charlie"),
						get_from_secret::<ImOnlineId>("//Charlie"),

					),
					(
						array_bytes::hex_n_into_unchecked(DOROTHY),
						get_from_secret::<AuraId>("//Dave"),
						get_from_secret::<GrandpaId>("//Dave"),
						get_from_secret::<ImOnlineId>("//Dave"),
					),
				],
				// Sudo account
				array_bytes::hex_n_into_unchecked(ALITH), 

				// Pre-funded accounts
				vec![
					array_bytes::hex_n_into_unchecked(ALITH),
					array_bytes::hex_n_into_unchecked(BALTATHAR),
					array_bytes::hex_n_into_unchecked(CHARLETH),
					array_bytes::hex_n_into_unchecked(DOROTHY),
				],
				true,
			)
		},
		vec![],
		None,
		None,
		None,
		Some(chainspec_properties()),
		None,
	))
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { aura, grandpa, im_online }
}

pub fn chainspec_properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("tokenSymbol".into(), "STOR".into());
	properties
}



pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![(
					array_bytes::hex_n_into_unchecked(ALITH),
					get_from_secret::<AuraId>("//Alice"), 
					get_from_secret::<GrandpaId>("//Alice"),
					get_from_secret::<ImOnlineId>("//Alice"),
				)],
				// Sudo account
				// AccountId::from(hex!("6B7CD45dfc550F12b4EdAFDFbBC68b53faAE6Fe2")),
				array_bytes::hex_n_into_unchecked(ALITH), 
				// Pre-funded accounts
				vec![
					AccountId::from(hex!("6B7CD45dfc550F12b4EdAFDFbBC68b53faAE6Fe2")),
					AccountId::from(hex!("18119Bb0f49ee709104CA2804B297B08d5d0EDEc")),
					AccountId::from(hex!("71B18c74b51E2195c92C169504f7FAFA71308A9a")),
					AccountId::from(hex!("C03cfc225Ad4b42F96f612BA38bD4d9cBD4a419a")),

				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(chainspec_properties()),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						get_from_secret::<AuraId>("//Alice"),
						get_from_secret::<GrandpaId>("//Alice"),
						get_from_secret::<ImOnlineId>("//Alice"),
					),
					(
						array_bytes::hex_n_into_unchecked(BALTATHAR),
						get_from_secret::<AuraId>("//Bob"),
						get_from_secret::<GrandpaId>("//Bob"),
						get_from_secret::<ImOnlineId>("//Bob"),
					),
				],
				// Sudo account
				array_bytes::hex_n_into_unchecked(ALITH), 
				// AccountId::from(hex!("6B7CD45dfc550F12b4EdAFDFbBC68b53faAE6Fe2")),
				// Pre-funded accounts
				vec![
					array_bytes::hex_n_into_unchecked(ALITH),
					array_bytes::hex_n_into_unchecked(BALTATHAR),
					array_bytes::hex_n_into_unchecked(CHARLETH),
					array_bytes::hex_n_into_unchecked(DOROTHY),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		Some(chainspec_properties()),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 5_000_000_000 * STOR;
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k.clone(), ENDOWMENT / endowed_accounts.len() as u128))
				.collect(),
		},

		validator_set: ValidatorSetConfig {
			initial_validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},

		// aura: Default::default(),
		aura: AuraConfig {
			// authorities: initial_authorities.iter().map(|x| (x.1.clone())).collect(),
			authorities : vec![], 
		},
		// grandpa: Default::default(),
		grandpa: GrandpaConfig {
			// authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
			authorities : vec![], 

		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		im_online: ImOnlineConfig {
			keys: vec![],
		},
		transaction_payment: Default::default(),
		evm: EVMConfig {
			accounts: {
				let mut accounts = BTreeMap::new();
				accounts.insert(
					H160::from_slice(&hex!("6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b")),
					GenesisAccount {
						nonce: U256::zero(),
						balance: Default::default(),
						code: vec![],
						storage: BTreeMap::new(),
					},
				);
				accounts
			},
		},

		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.1.clone(), x.2.clone(), x.3.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},


		ethereum: Default::default(),
		base_fee: Default::default(),
	}
}
 
 