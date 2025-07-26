use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_core::{Pair, Public, H256, sr25519, ed25519, crypto::{Ss58Codec, AccountId32}};
use sc_network::{config::MultiaddrWithPeerId, PeerId};
use oslo_network_runtime::{currency::*, opaque::SessionKeys, WASM_BINARY,
	CouncilConfig, DemocracyConfig, ValidatorSetConfig, SS58Prefix, Signature,
	TechnicalCommitteeConfig, TreasuryConfig, TransactionPaymentConfig,
	EVMConfig, EthereumConfig, BaseFeeConfig, ImOnlineConfig, BalancesConfig,
	GrandpaConfig, AuraConfig, SudoConfig, SystemConfig, SessionConfig, AccountId}; 
use std::{default::Default};
/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;
use sp_runtime::traits::{IdentifyAccount, Verify};
/// Helper function to generate a crypto pair from seed
fn get_from_secret<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None).unwrap_or_else(|_| panic!("Invalid string '{}'", seed)).public()
}

type AccountPublic = <Signature as Verify>::Signer;


pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_secret::<TPublic>(seed)).into_account()
}


pub fn chainspec_properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 6.into());
	properties.insert("tokenSymbol".into(), "OSLO".into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());
	properties
}

/// The initial coin supply given to the endowed accounts. 
const INITIALSUPPLY: u128 = 49_999_999_900_000;
/// Mainnet node 4 has remainder of initial supply for chain development purposes only
const NODE4SUPPLY: u128 = 200_000;

/// The H160 addresses below are all actually shortened Grandpa hex addresses

/// Begin Testnet Addresses

const TESTNETNODE1AURA: &str = "5EP15U8PitEa8N2qrKaCrafZzyp3RsU59owdv5mWhWtSM1Mc";
const TESTNETNODE1AURAHEX: &str = "0x666cdaab93a8fb6ffba8457fe9d6a5ff9704d3c359403cbad7c9f633a714c74f";
const TESTNETNODE1GRANDPA: &str = "5DDAPHxEebdeaKUysxudLHteRkUXdmi14koJANS2DsanZyoi";
const TESTNETNODE1GRANDPAHEX: &str = "0x32af7f7772b33a64c0382ddeae52c0f33bae7df2c7f2d91a3246ca3e49cdcbd2";
const TESTNETNODE1H160: &str = "0x32af7f7772b33a64c0382ddeae52c0f33bae7df2";

//PeerId 12D3KooWCreP3P122JcyiczGqt4pBZqyNX7w6YcNhtRVMD7mzB74
const TESTNETNODE2AURA: &str = "5DUNUSVj3fN9iWxaqSbM7bmdecnRUu9Jxs4Uq5BwqFQbENbo";
const TESTNETNODE2AURAHEX: &str = "0x3e48df9289ff7f5fc1d7c3b61db8a61fd77d75a5b02c0b1d68fa4a5fc9ec5002";
const TESTNETNODE2GRANDPA: &str = "5D5v4ND8soXKD241Szoswcv6aBY9XbemgxJSE3DB8jVsAdxZ";
const TESTNETNODE2GRANDPAHEX: &str = "0x2d288dab0ab0edbbca2bdb741ec430b4cc6e8ba14683051e7acf16721d44bd2f";
const TESTNETNODE2H160: &str = "0x2d288dab0ab0edbbca2bdb741ec430b4cc6e8ba1";

///Begin Mainnet Addresses
const MAINNETSUDONODEAURA: &str = "5HTJDxz8GCjFtaYj5HWffMYBFxhK5Sk7H9LicEATkcznYEEr";
const MAINNETSUDONODEAURAHEX: &str = "0xee6841a6e1e6b126c19df8673edbbd5967e20a22c79fbe92cf81949c11641877";
const MAINNETSUDONODEGRANDPA: &str = "5DrCUaDk4i1iLyReuoRMQ3GjPGRUDLT3vnjvpMZbheFT2tpm";
const MAINNETSUDONODEGRANDPAHEX: &str = "0x4eee9df0be0893a22410520825cc1a1a0dea0a7be08e7782c30acd70c137fbd6";
const MAINNETSUDONODEH160: &str = "0x4eee9df0be0893a22410520825cc1a1a0dea0a7b";


const MAINNETNODE1AURA: &str = "5DNz3Ki95vHZhwrb4fy742BPgxnmFj7jRZ1CQQySe7vwvhoA";
const MAINNETNODE1AURAHEX: &str = "0x3a2d2142b7d16b1ad0c9ab32aa150041f9d8257babfc289c5b9b988b4d70de50";
const MAINNETNODE1GRANDPA: &str = "5EFf4UQZWAyVsthoXSAmrYPESNPcuJVTwKaieSPNKuvsNkg9";
const MAINNETNODE1GRANDPAHEX: &str = "0x60d2c0a998720f03bb4ef9ed8201d89473fef3e1f22d380fdf4e93d47f9b7d36";
const MAINNETNODE1H160: &str = "0x60d2c0a998720f03bb4ef9ed8201d89473fef3e1";


const MAINNETNODE2AURA: &str = "5HgMQRHFm7bGS3ZBALDNtwJS5whD3iBWAuAmZeVp994UcDkv";
const MAINNETNODE2AURAHEX: &str = "0xf85d26fe68c7a627fa4c1b7b80c2d4907f45e445abb58287c0abd647b6308f69";
const MAINNETNODE2GRANDPA: &str = "5HKVESAv3hQYVgqnRYbmGYruEEXAbgZ1hrJemysgN1uUXCfy";
const MAINNETNODE2GRANDPAHEX: &str = "0xe873598775fcaae94166583bdb93c1dc2f1d83a93579cafe6eb9434b9aee819d";
const MAINNETNODE2H160: &str = "0xe873598775fcaae94166583bdb93c1dc2f1d83a9";


const MAINNETNODE3AURA: &str = "5DSFVPUfvxMnpFKWjHEukJcPCt4ER15RpDUN1hE5e5jAee4s";
const MAINNETNODE3AURAHEX: &str = "0x3caadf99c26f79d251096ffee2aad7fab8bb08c4c59426fcbd43c1a0ab91c441";
const MAINNETNODE3GRANDPA: &str = "5Gkt96oLmHWKeK7cAvq9gfegQpcDpDs63yX8csEw9BcFc5nr";
const MAINNETNODE3GRANDPAHEX: &str = "0xcf955ddfe3874a306fc9fa4be2983e3157e24be9da722ba3477d6413ed39ab4f";
const MAINNETNODE3H160: &str = "0xcf955ddfe3874a306fc9fa4be2983e3157e24be9";

//PeerId 12D3KooWCreP3P122JcyiczGqt4pBZqyNX7w6YcNhtRVMD7mzB74
const MAINNETNODE4AURA: &str = "5DUNUSVj3fN9iWxaqSbM7bmdecnRUu9Jxs4Uq5BwqFQbENbo";
const MAINNETNODE4AURAHEX: &str = "0x3e48df9289ff7f5fc1d7c3b61db8a61fd77d75a5b02c0b1d68fa4a5fc9ec5002";
const MAINNETNODE4GRANDPA: &str = "5D5v4ND8soXKD241Szoswcv6aBY9XbemgxJSE3DB8jVsAdxZ";
const MAINNETNODE4GRANDPAHEX: &str = "0x2d288dab0ab0edbbca2bdb741ec430b4cc6e8ba14683051e7acf16721d44bd2f";
const MAINNETNODE4H160: &str = "0x2d288dab0ab0edbbca2bdb741ec430b4cc6e8ba1";


//PeerId 12D3KooWDcXdEcTyqePJYCKvduX6vt3qLhjQWieg5FUkPJmkJML2
const MAINNETNODE5AURA: &str = "5GuoZBbAjUQxJyjAXHiQ4yXDHFSJY6Z9fQAYuwhrvbX5NcWi";
const MAINNETNODE5AURAHEX: &str = "0xd663243eb04c28df055ff070abebcc7c4705b5a798b032521bc4ce0a91388b68";
const MAINNETNODE5GRANDPA: &str = "5DLeyjCzSB7o1DfbhZmy5hNpG2FJPyMqd4XSz5w1A8276YKA";
const MAINNETNODE5GRANDPAHEX: &str = "0x38667884513ea7b233411b2d5b3e9fef3bdce8b052ee595492af9abae441f337";
const MAINNETNODE5H160: &str = "0x38667884513ea7b233411b2d5b3e9fef3bdce8b0";

pub fn public_config() -> Result<ChainSpec, String> {
	let mainnetsudonodepeerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(MAINNETSUDONODEGRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let mainnetnode1peerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(MAINNETNODE1GRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let mainnetnode2peerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(MAINNETNODE2GRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let mainnetnode3peerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(MAINNETNODE3GRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let mainnetnode4peerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(MAINNETNODE4GRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let mainnetnode5peerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(MAINNETNODE5GRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let mainnet_sudo_node: MultiaddrWithPeerId = ("/dns/sudonode.oslocrypto.com/tcp/37954/p2p/".to_owned() + &mainnetsudonodepeerid.to_string()).parse().unwrap();
	let mainnet_boot_node_1: MultiaddrWithPeerId = ("/dns/node1.oslocrypto.com/tcp/37954/p2p/".to_owned() + &mainnetnode1peerid.to_string()).parse().unwrap();
	let mainnet_boot_node_2: MultiaddrWithPeerId = ("/dns/node2.oslocrypto.com/tcp/37954/p2p/".to_owned() + &mainnetnode2peerid.to_string()).parse().unwrap();
	let mainnet_boot_node_3: MultiaddrWithPeerId = ("/dns/node3.oslocrypto.com/tcp/37954/p2p/".to_owned() + &mainnetnode3peerid.to_string()).parse().unwrap();
	let mainnet_boot_node_4: MultiaddrWithPeerId = ("/dns/node4.oslocrypto.com/tcp/37954/p2p/".to_owned() + &mainnetnode4peerid.to_string()).parse().unwrap();
	let mainnet_boot_node_5: MultiaddrWithPeerId = ("/dns/node5.oslocrypto.com/tcp/37954/p2p/".to_owned() + &mainnetnode5peerid.to_string()).parse().unwrap();
	let mainnet_boot_nodes_vec: Vec<MultiaddrWithPeerId> = vec![mainnet_sudo_node, mainnet_boot_node_1, mainnet_boot_node_2, mainnet_boot_node_3, mainnet_boot_node_4, mainnet_boot_node_5];
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default()).with_boot_nodes(mainnet_boot_nodes_vec).with_name("Oslo-Network-Mainnet")
		.with_protocol_id("Oslo-Network-Mainnet").with_id("Oslo-Network_Mainnet").with_chain_type(ChainType::Live).with_properties(chainspec_properties())
		.with_genesis_config_patch(mainnet_genesis(
			vec![ // Initial PoA authorities
				(
					array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETSUDONODEGRANDPAHEX),
					sr25519::Public::from_h256(MAINNETSUDONODEAURAHEX.parse::<H256>().unwrap()).into(),
					ed25519::Public::from_h256(MAINNETSUDONODEGRANDPAHEX.parse::<H256>().unwrap()).into(),
					sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(MAINNETSUDONODEAURA).unwrap().0.as_ref()).unwrap()).into()
				),
				(
					array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETNODE1GRANDPAHEX),
					sr25519::Public::from_h256(MAINNETNODE1AURAHEX.parse::<H256>().unwrap()).into(),
					ed25519::Public::from_h256(MAINNETNODE1GRANDPAHEX.parse::<H256>().unwrap()).into(),
					sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(MAINNETNODE1AURA).unwrap().0.as_ref()).unwrap()).into()
				),
				(
					array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETNODE2GRANDPAHEX),
					sr25519::Public::from_h256(MAINNETNODE2AURAHEX.parse::<H256>().unwrap()).into(),
					ed25519::Public::from_h256(MAINNETNODE2GRANDPAHEX.parse::<H256>().unwrap()).into(),
					sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(MAINNETNODE2AURA).unwrap().0.as_ref()).unwrap()).into()
				),
				(
					array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETNODE3GRANDPAHEX),
					sr25519::Public::from_h256(MAINNETNODE3AURAHEX.parse::<H256>().unwrap()).into(),
					ed25519::Public::from_h256(MAINNETNODE3GRANDPAHEX.parse::<H256>().unwrap()).into(),
					sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(MAINNETNODE3AURA).unwrap().0.as_ref()).unwrap()).into()
				),
				(
					array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETNODE4GRANDPAHEX),
					sr25519::Public::from_h256(MAINNETNODE4AURAHEX.parse::<H256>().unwrap()).into(),
					ed25519::Public::from_h256(MAINNETNODE4GRANDPAHEX.parse::<H256>().unwrap()).into(),
					sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(MAINNETNODE4AURA).unwrap().0.as_ref()).unwrap()).into()
				),
				(
					array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETNODE5GRANDPAHEX),
					sr25519::Public::from_h256(MAINNETNODE5AURAHEX.parse::<H256>().unwrap()).into(),
					ed25519::Public::from_h256(MAINNETNODE5GRANDPAHEX.parse::<H256>().unwrap()).into(),
					sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(MAINNETNODE5AURA).unwrap().0.as_ref()).unwrap()).into()
				)
			],
			// Sudo account
			array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETSUDONODEGRANDPAHEX),
			// Pre-funded accounts
			vec![
				array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETSUDONODEGRANDPAHEX),
				array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETNODE1GRANDPAHEX)
			],
			true
	)).build())
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	let testnetnode1peerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(TESTNETNODE1GRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let testnetnode2peerid: PeerId = sc_network::config::ed25519::PublicKey::try_from_bytes(AccountId32::from_ss58check_with_version(TESTNETNODE2GRANDPA).unwrap().0.as_ref()).unwrap().to_peer_id().into();
	let boot_node_1: MultiaddrWithPeerId = ("/dns/testnetnode1.oslocrypto.com/tcp/30333/p2p/".to_owned() + &testnetnode1peerid.to_string()).parse().unwrap();
	let boot_node_2: MultiaddrWithPeerId = ("/dns/testnetnode2.oslocrypto.com/tcp/30333/p2p/".to_owned() + &testnetnode2peerid.to_string()).parse().unwrap();
	let boot_nodes_vec: Vec<MultiaddrWithPeerId> = vec![boot_node_1, boot_node_2];
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default()).with_boot_nodes(boot_nodes_vec)
	.with_name("Oslo-Network-Testnet").with_protocol_id("Oslo-Network-Testnet").with_id("Oslo-Network_Testnet").with_chain_type(ChainType::Local).with_properties(chainspec_properties())
	.with_genesis_config_patch(testnet_genesis(
		// Initial PoA authorities
		vec![
			(
				array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(TESTNETNODE1GRANDPAHEX),
				sr25519::Public::from_h256(TESTNETNODE1AURAHEX.parse::<H256>().unwrap()).into(),
				ed25519::Public::from_h256(TESTNETNODE1GRANDPAHEX.parse::<H256>().unwrap()).into(),
				sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(TESTNETNODE1AURA).unwrap().0.as_ref()).unwrap()).into()
			),
			(
				array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(TESTNETNODE2GRANDPAHEX),
				sr25519::Public::from_h256(TESTNETNODE2AURAHEX.parse::<H256>().unwrap()).into(),
				ed25519::Public::from_h256(TESTNETNODE2GRANDPAHEX.parse::<H256>().unwrap()).into(),
				sr25519::Public::from_raw(<[u8; 32]>::try_from(AccountId32::from_ss58check_with_version(TESTNETNODE2AURA).unwrap().0.as_ref()).unwrap()).into()
			)
		],
		// Sudo account
		array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(TESTNETNODE1GRANDPAHEX),
		// Pre-funded accounts
		vec![
			array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(TESTNETNODE1GRANDPAHEX),
			array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(TESTNETNODE2GRANDPAHEX)
		],
		true
	)).build())
}

pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(WASM_BINARY.expect("WASM not available"), Default::default()).with_name("Oslo-Network-Dev")
	.with_id("dev").with_chain_type(ChainType::Development).with_properties(chainspec_properties())
	.with_genesis_config_patch(testnet_genesis(
		// Initial PoA authorities
		vec![(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_from_secret::<AuraId>("//Alice"),
			get_from_secret::<GrandpaId>("//Alice"),
			get_from_secret::<ImOnlineId>("//Alice")
		)],
		// Sudo account
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		// Pre-funded accounts
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
		],
		true
	)).build())
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys { SessionKeys { aura, grandpa, im_online } }

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool
) -> serde_json::Value {
	let num_endowed_accounts = endowed_accounts.len();
	serde_json::json!({
		"system": SystemConfig{ _config: Default::default() },
		"balances": BalancesConfig{ balances: endowed_accounts.iter().cloned().map(|k| (k, INITIALSUPPLY * OSLO)).collect::<Vec<_>>()},
		"democracy": DemocracyConfig::default(),
		"validatorSet": ValidatorSetConfig{initial_validators: initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>()},
		"session": SessionConfig{ keys: initial_authorities.into_iter().map(|(acc, aura, gran, im_online)| {
			(acc.clone(), acc.clone(), session_keys(aura.clone(), gran.clone(), im_online.clone()))}).collect::<Vec<_>>()},
		"ethereum": EthereumConfig::default(),
		"baseFee": BaseFeeConfig::default(),
		"imOnline": ImOnlineConfig { keys: vec![] },
		"aura": AuraConfig{ authorities: vec![] },
		"grandpa": GrandpaConfig{ _config: Default::default(), authorities: vec![] },
		"sudo": SudoConfig{ key: Some(root_key) },
		"treasury": TreasuryConfig::default(),
		"transactionPayment": TransactionPaymentConfig::default(),
		"evm" : EVMConfig::default(),
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
	let num_endowed_accounts = endowed_accounts.len() + 1; //counting node 4
	serde_json::json!({
		"system": SystemConfig{ _config: Default::default() },
		"balances": BalancesConfig{ 
			balances: {
				let mut balances_vec = endowed_accounts.iter().cloned().map(|k| (k, INITIALSUPPLY * OSLO)).collect::<Vec<_>>();
		        balances_vec.push((array_bytes::hex_n_into_unchecked::<&str, sp_runtime::AccountId32, 32>(MAINNETNODE4GRANDPAHEX), NODE4SUPPLY * OSLO));
		        balances_vec
			}
		},
		"democracy": DemocracyConfig::default(),
		"validatorSet": ValidatorSetConfig{initial_validators: initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>()},
		"session": SessionConfig{ keys: initial_authorities.into_iter().map(|(acc, aura, gran, im_online)| {
			(acc.clone(), acc.clone(), session_keys(aura.clone(), gran.clone(), im_online.clone()))}).collect::<Vec<_>>()},
		"ethereum": EthereumConfig::default(),
		"baseFee": BaseFeeConfig::default(),
		"imOnline": ImOnlineConfig { keys: vec![] },
		"aura": AuraConfig{ authorities: vec![] },
		"grandpa": GrandpaConfig{ _config: Default::default(), authorities: vec![] },
		"sudo": SudoConfig{ key: Some(root_key) },
		"treasury": TreasuryConfig::default(),
		"transactionPayment": TransactionPaymentConfig::default(),
		"evm" : EVMConfig::default(),
		"council": CouncilConfig::default(),
		"technicalCommittee": TechnicalCommitteeConfig {
			members: endowed_accounts.iter().take((num_endowed_accounts + 1) / 2).cloned().collect(),
			phantom: Default::default()
		}
	})
}