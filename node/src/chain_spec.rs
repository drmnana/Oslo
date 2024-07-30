use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_core::{Pair, Public};
//use pallet_collective::GenesisConfig as CollectiveConfig;
use oslo_network_runtime::{
	currency::*, opaque::SessionKeys, AccountId, WASM_BINARY,
	CouncilConfig, DemocracyConfig, RuntimeGenesisConfig,
	TechnicalCommitteeConfig, TreasuryConfig, TransactionPaymentConfig,
	EVMConfig, EthereumConfig, BaseFeeConfig, ImOnlineConfig, BalancesConfig,
	GrandpaConfig, AuraConfig, SudoConfig, SystemConfig, Signature,
	SessionConfig, ValidatorSetConfig
};
use std::default::Default;
/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

use sp_runtime::traits::Verify;
type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
fn get_from_secret<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None).unwrap_or_else(|_| panic!("Invalid string '{}'", seed)).public()
}
pub fn chainspec_properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("tokenSymbol".into(), "OSLO".into());
	properties
}

const ALITH: &str = "0x83451391e196556A66ebfCe472165d37E6575F5e";
const BALTATHAR: &str = "0xB78ef962F15Fb30d70fE7f5e00aA042869a2293A";
const CHARLETH: &str = "0x8ff34400aAb1Ee14Ee26BE799e107BDAAD88df8d";
const DOROTHY: &str = "0x8cBcD18e730eFD8c9C26e6791455CD100477DAEe";

pub fn public_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
		.with_name("OsloNetwork Mainnet").with_id("public_live").with_chain_type(ChainType::Live).with_properties(chainspec_properties())
		.with_genesis_config_patch(mainnet_genesis(
			// Initial PoA authorities
			vec![
				(
					array_bytes::hex_n_into_unchecked(ALITH),
					get_from_secret::<AuraId>("//Alice"),
					get_from_secret::<GrandpaId>("//Alice"),
					get_from_secret::<ImOnlineId>("//Alice")
				)
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
			true
		)).build())
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { aura, grandpa, im_online }
}


pub fn testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
	.with_name("Oslo-Network Testnet").with_id("Oslo-Network_Testnet").with_chain_type(ChainType::Local).with_properties(chainspec_properties())
	.with_genesis_config_patch(testnet_genesis(
		// Initial PoA authorities
		vec![
			(
				array_bytes::hex_n_into_unchecked(ALITH),
				get_from_secret::<AuraId>("//Alice"),
				get_from_secret::<GrandpaId>("//Alice"),
				get_from_secret::<ImOnlineId>("//Alice")
			)
		],
		// Sudo account
		array_bytes::hex_n_into_unchecked(ALITH),
		// Pre-funded accounts
		vec![
			array_bytes::hex_n_into_unchecked(ALITH),
			array_bytes::hex_n_into_unchecked(BALTATHAR),
			array_bytes::hex_n_into_unchecked(CHARLETH),
			array_bytes::hex_n_into_unchecked(DOROTHY)
		],
		true
	)).build())
}

pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
	.with_name("Oslo-Network Testnet").with_id("dev").with_chain_type(ChainType::Development).with_properties(chainspec_properties())
	.with_genesis_config_patch(testnet_genesis(
			// Initial PoA authorities
			vec![
				(
					array_bytes::hex_n_into_unchecked(ALITH),
					get_from_secret::<AuraId>("//Alice"),
					get_from_secret::<GrandpaId>("//Alice"),
					get_from_secret::<ImOnlineId>("//Alice")
				)
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
			true
		)).build())
}

// /// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool
) -> serde_json::Value {
	let num_endowed_accounts = endowed_accounts.len();
	serde_json::json!({
		"system": SystemConfig{ _config: Default::default() },
		"sudo": SudoConfig{ key: Some(root_key) },
		"balances": BalancesConfig{ balances: endowed_accounts.iter().cloned().map(|k| (k, 1_000_000 * OSLO)).collect::<Vec<_>>()},
		"aura": AuraConfig{ authorities: vec![] },
		"grandpa": GrandpaConfig{ _config: Default::default(), authorities: vec![] },
		"democracy": DemocracyConfig::default(),
		"validatorSet": ValidatorSetConfig{initial_validators: initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>()},
		"session": SessionConfig{ keys: initial_authorities.into_iter().map(|(acc, aura, gran, im_online)| {
			(acc.clone(), acc, session_keys(aura, gran, im_online))}).collect::<Vec<_>>()},
		"ethereum": EthereumConfig::default(),
		"baseFee": BaseFeeConfig::default(),
		"imOnline": ImOnlineConfig { keys: vec![] },
		"treasury": TreasuryConfig::default(),
		"transactionPayment": TransactionPaymentConfig::default(),
		"evm" : EVMConfig::default(),
		"democracy": DemocracyConfig::default(),
		"council": CouncilConfig::default(),
		"technicalCommittee": TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
	})
}

fn mainnet_genesis(
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool
) -> serde_json::Value {
	let num_endowed_accounts = endowed_accounts.len();
	serde_json::json!({
		"system": SystemConfig{ _config: Default::default() },
		"sudo": SudoConfig{ key: Some(root_key) },
		"balances": BalancesConfig{ balances: endowed_accounts.iter().cloned().map(|k| (k, 1_000_000 * OSLO)).collect::<Vec<_>>()},
		"aura": AuraConfig{ authorities: vec![] },
		"grandpa": GrandpaConfig{ _config: Default::default(), authorities: vec![]},
		"democracy": <DemocracyConfig as std::default::Default>::default(),
		"validatorSet": ValidatorSetConfig{initial_validators: initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>()},
		"session": SessionConfig{ keys: initial_authorities.into_iter().map(|(acc, aura, gran, im_online)| {
			(acc.clone(), acc, session_keys(aura, gran, im_online))}).collect::<Vec<_>>()},
		"ethereum": EthereumConfig::default(),
		"baseFee": BaseFeeConfig::default(),
		"imOnline": ImOnlineConfig { keys: vec![] },
		"treasury": TreasuryConfig::default(),
		"transactionPayment": TransactionPaymentConfig::default(),
		"evm" : EVMConfig::default(),
		"democracy": DemocracyConfig::default(),
		"council": CouncilConfig::default(),
		"technicalCommittee": TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default()
		}
	})
}
