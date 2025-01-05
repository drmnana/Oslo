use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_core::{Pair, Public};
use sp_core::{sr25519, ed25519};
use sc_network::{config::MultiaddrWithPeerId};

//use pallet_collective::GenesisConfig as CollectiveConfig;
use oslo_network_runtime::{
	currency::*, opaque::SessionKeys, AccountId, WASM_BINARY,
	CouncilConfig, DemocracyConfig, RuntimeGenesisConfig, ValidatorSetConfig,
	TechnicalCommitteeConfig, TreasuryConfig, TransactionPaymentConfig,
	EVMConfig, EthereumConfig, BaseFeeConfig, ImOnlineConfig, BalancesConfig,
	GrandpaConfig, AuraConfig, SudoConfig, SystemConfig, SessionConfig
};
use std::default::Default;
use sp_core::H256;
/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;
use sp_runtime::traits::Verify;
type AccountPublic = <oslo_network_runtime::Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
fn get_from_secret<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None).unwrap_or_else(|_| panic!("Invalid string '{}'", seed)).public()
}

/// Generate a crypto pair from seed.
/*pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}*/

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

const JOSEY: &str = "0xd1fd49da79e79af0Ac5A0C4c1658E9fE1cb076E6";
const TESTNETNODEH160: &str = "0xd15c340c69a3a09f1472b8c9eb3a3e8bf9e3abe5";
const TESTNETNODEAURA: &str = "5FxfxpKVcydFo3UdGyGxVPHLF8CdCZ7KMYvaeYrakKe8RPHw";
const TESTNETNODEGRANDPA: &str = "5GoDFkeEL5sYubk7XmWk1a148yS3nrZasWgCPR3sXVNYgMjf";
const TESTNETNODEAURAHEX: &str = "0xac5715ea68463a03cf360fcfc79b95435897f9c7252efc5315c3c3c4eab7013c";
const TESTNETNODEGRANDPAHEX: &str = "0xd15c340c69a3a09f1472b8c9eb3a3e8bf9e3abe50f3ef36cb96908e480c5c9fb";

const TESTNETNODE2H160: &str = "0x32af7f7772b33a64c0382ddeae52c0f33bae7df2";
const TESTNETNODE2AURA: &str = "5EP15U8PitEa8N2qrKaCrafZzyp3RsU59owdv5mWhWtSM1Mc";
const TESTNETNODE2GRANDPA: &str = "5DDAPHxEebdeaKUysxudLHteRkUXdmi14koJANS2DsanZyoi";
const TESTNETNODE2AURAHEX: &str = "0x666cdaab93a8fb6ffba8457fe9d6a5ff9704d3c359403cbad7c9f633a714c74f";
const TESTNETNODE2GRANDPAHEX: &str = "0x32af7f7772b33a64c0382ddeae52c0f33bae7df2c7f2d91a3246ca3e49cdcbd2";

pub fn public_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default())
		.with_name("OsloNetwork Mainnet").with_id("public_live").with_chain_type(ChainType::Live).with_properties(chainspec_properties())
		.with_genesis_config_patch(mainnet_genesis(
			// Initial PoA authorities
			vec![
				(
					array_bytes::hex_n_into_unchecked(JOSEY),
					get_from_secret::<AuraId>("//Alice"),
					get_from_secret::<GrandpaId>("//Alice"),
					get_from_secret::<ImOnlineId>("//Alice")
				)
			],
			// Sudo account
			array_bytes::hex_n_into_unchecked(JOSEY),
			// Pre-funded accounts
			vec![
				array_bytes::hex_n_into_unchecked(ALITH),
				array_bytes::hex_n_into_unchecked(BALTATHAR),
				array_bytes::hex_n_into_unchecked(CHARLETH),
				array_bytes::hex_n_into_unchecked(DOROTHY),
				array_bytes::hex_n_into_unchecked(JOSEY)
			],
			true
		)).build())
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { aura, grandpa, im_online }
}


pub fn testnet_config() -> Result<ChainSpec, String> {
	let boot_node_1: MultiaddrWithPeerId = "/dns/node1.oslocrypto.com/tcp/30334/p2p/12D3KooWFZ3BqJh4K8SsB3GnTQeigzmFfxXMP4f3EMBnTeEXRD2V".parse().unwrap();
	let boot_node_2: MultiaddrWithPeerId = "/dns/node2.oslocrypto.com/tcp/30333/p2p/12D3KooWHzevxCW8b6Ck4GMsTNX1tkPWF2eexsERAbBhfPSH8PPz".parse().unwrap();
	let boot_nodes_vec: Vec<MultiaddrWithPeerId> = vec![boot_node_1, boot_node_2];
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default()).with_boot_nodes(boot_nodes_vec)
	.with_name("Oslo-Network Testnet").with_protocol_id("Oslo-Network-Testnet").with_id("Oslo-Network_Testnet").with_chain_type(ChainType::Local).with_properties(chainspec_properties())
	.with_genesis_config_patch(testnet_genesis(
		// Initial PoA authorities
		vec![
			(
				array_bytes::hex_n_into_unchecked(TESTNETNODEH160),
				sr25519::Public::from_h256(TESTNETNODEAURAHEX.parse::<H256>().unwrap()).into(),
				ed25519::Public::from_h256(TESTNETNODEGRANDPAHEX.parse::<H256>().unwrap()).into(),
				sr25519::Public::from_h256(TESTNETNODEAURAHEX.parse::<H256>().unwrap()).into()
			),
			(
				array_bytes::hex_n_into_unchecked(TESTNETNODE2H160),
				sr25519::Public::from_h256(TESTNETNODE2AURAHEX.parse::<H256>().unwrap()).into(),
				ed25519::Public::from_h256(TESTNETNODE2GRANDPAHEX.parse::<H256>().unwrap()).into(),
				sr25519::Public::from_h256(TESTNETNODE2AURAHEX.parse::<H256>().unwrap()).into()
			)
		],
		// Sudo account
		array_bytes::hex_n_into_unchecked(TESTNETNODEH160),
		// Pre-funded accounts
		vec![
			array_bytes::hex_n_into_unchecked(ALITH),
			array_bytes::hex_n_into_unchecked(BALTATHAR),
			array_bytes::hex_n_into_unchecked(CHARLETH),
			array_bytes::hex_n_into_unchecked(DOROTHY),
			array_bytes::hex_n_into_unchecked(JOSEY),
			array_bytes::hex_n_into_unchecked(TESTNETNODEH160),
			array_bytes::hex_n_into_unchecked(TESTNETNODE2H160)
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
					array_bytes::hex_n_into_unchecked(JOSEY),
					get_from_secret::<AuraId>("//Alice"),
					get_from_secret::<GrandpaId>("//Alice"),
					get_from_secret::<ImOnlineId>("//Alice")
				)
			],
			// Sudo account
			array_bytes::hex_n_into_unchecked(JOSEY),
			// Pre-funded accounts
			vec![
				array_bytes::hex_n_into_unchecked(ALITH),
				array_bytes::hex_n_into_unchecked(BALTATHAR),
				array_bytes::hex_n_into_unchecked(CHARLETH),
				array_bytes::hex_n_into_unchecked(DOROTHY),
				array_bytes::hex_n_into_unchecked(JOSEY)
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
		"democracy": DemocracyConfig::default(),
		"validatorSet": ValidatorSetConfig{initial_validators: initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>()},
		"session": SessionConfig{ keys: initial_authorities.into_iter().map(|(acc, aura, gran, im_online)| {
			(acc.clone(), acc.clone(), session_keys(aura.clone(), gran.clone(), im_online.clone()))}).collect::<Vec<_>>()},
		"ethereum": EthereumConfig::default(),
		"baseFee": BaseFeeConfig::default(),
		"imOnline": ImOnlineConfig { keys: vec![] },
		"aura": AuraConfig{ authorities: vec![] },
		"grandpa": GrandpaConfig{ _config: Default::default(), authorities: vec![] },
		"treasury": TreasuryConfig::default(),
		"transactionPayment": TransactionPaymentConfig::default(),
		"evm" : EVMConfig::default(),
		"democracy": DemocracyConfig::default(),
		"council": CouncilConfig::default(),
		"technicalCommittee": TechnicalCommitteeConfig {
			members: endowed_accounts.iter().take((num_endowed_accounts + 1) / 2).cloned().collect(),
			phantom: Default::default()
		}
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
			(acc.clone(), acc.clone(), session_keys(aura, gran, im_online))}).collect::<Vec<_>>()},
		"ethereum": EthereumConfig::default(),
		"baseFee": BaseFeeConfig::default(),
		"imOnline": ImOnlineConfig { keys: vec![] },
		"treasury": TreasuryConfig::default(),
		"transactionPayment": TransactionPaymentConfig::default(),
		"evm" : EVMConfig::default(),
		"democracy": DemocracyConfig::default(),
		"council": CouncilConfig::default(),
		"technicalCommittee": TechnicalCommitteeConfig {
			members: endowed_accounts.iter().take((num_endowed_accounts + 1) / 2).cloned().collect(),
			phantom: Default::default()
		}
	})
}
