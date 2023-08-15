use hex_literal::hex;
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
// use sp_runtime::key_types::IM_ONLINE;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_core::crypto::UncheckedInto;

use storage_chain_runtime::{
	currency::*, opaque::SessionKeys, AccountId, AuraConfig, BalancesConfig,
	CouncilConfig, DemocracyConfig, GenesisConfig, GrandpaConfig,
	ImOnlineConfig, SessionConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	ValidatorSetConfig, WASM_BINARY,
};
use std::{ default::Default};

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

const ALITH: &str = "0x9643B6d120345604D1E93133C6a9eFd8cB5F6181";
const BALTATHAR: &str = "0x90E79DAc498b35096d4d86CEa4f2c3681b40F5C7";
const CHARLETH: &str = "0x6a321b74936ccA0F549FEF65F274c9E679258307";
const DOROTHY: &str = "0x71599dEdfEc2CE347a804F9bbf9d18C6C2D7009E";

pub fn public_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"StorageChain Mainnet",
		"public_live",
		ChainType::Live,
		move || {
			mainnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						hex!["2424bc9ceb258a8f74aa3f99d0211b51e88d16882754d43dc6511c1a5aea5b46"].unchecked_into(),
						hex!["67be3d9095d1275ede2e30795938707e5cf407851a7e9d673d8764d09fc8f583"].unchecked_into(),
						hex!["2424bc9ceb258a8f74aa3f99d0211b51e88d16882754d43dc6511c1a5aea5b46"].unchecked_into(),
					),
					(
						array_bytes::hex_n_into_unchecked(BALTATHAR),
						hex!["7851ca2f417fcaa8fb6c5e6e05c1f2f139d31dbdbe501a98432e46d7930f2431"].unchecked_into(),
						hex!["5f7d7a9ca599a6400ee8b7564174b97b14cbd7f4e7d4a42ace1a6f33de1bc115"].unchecked_into(),
						hex!["7851ca2f417fcaa8fb6c5e6e05c1f2f139d31dbdbe501a98432e46d7930f2431"].unchecked_into(),					),
					(	
						array_bytes::hex_n_into_unchecked(CHARLETH),
						hex!["e0b952dc8556d7a5fbfe538e8fb2adb445989e12f6c9e97748eaf5df982d640c"].unchecked_into(),
						hex!["5e7eb458379855b566d8ebeee738f544392075135f98980de7c63dcd46a19877"].unchecked_into(),
						hex!["e0b952dc8556d7a5fbfe538e8fb2adb445989e12f6c9e97748eaf5df982d640c"].unchecked_into(),					),
					(
						array_bytes::hex_n_into_unchecked(DOROTHY),
						hex!["cc4346263d27a5eb0f1bcbd9e53ab218b7e56af34951e1459334515c0dcacf70"].unchecked_into(),
						hex!["71f7dfca2808730aa2c5fb4233cc9600f060bcf5842025b878ebb4614db863d6"].unchecked_into(),
						hex!["cc4346263d27a5eb0f1bcbd9e53ab218b7e56af34951e1459334515c0dcacf70"].unchecked_into(),					),
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
		vec![
			"/ip4/167.71.244.181/tcp/30333/p2p/12D3KooWR99vJeAwvtFfQHRCsCWvB1Q8Ay9mX3raohLtqQyJho55".parse().unwrap(),
			"/ip4/157.230.233.32/tcp/30333/p2p/12D3KooWM64C9JqTVBE4xm4FNq4u1XZfR1bG7uKLcYBSr1ouLF1e".parse().unwrap(),
			"/ip4/134.209.121.166/tcp/30335/p2p/12D3KooWS5qssUFhyqMGBFMjvidyMpZ9H7RK2LqpTEp9qtjTaLsB".parse().unwrap(),
		],
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
		// Names
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
		"StorageChain Testnet",
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
		// Bootnodes
		vec![

		],
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
	let num_endowed_accounts = endowed_accounts.len();
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| {
					if k == array_bytes::hex_n_into_unchecked(BALTATHAR) {
						(k.clone(), 1_755_000_000 * STOR)
					} else if k == array_bytes::hex_n_into_unchecked(CHARLETH) {
						(k.clone(), 66_000_000 * STOR)
					} else if k == array_bytes::hex_n_into_unchecked(DOROTHY) {
						(k.clone(), 194_999_000 * STOR)
					} else if k == array_bytes::hex_n_into_unchecked(ALITH) {
						(k.clone(), 1000 * STOR)
					} else {
						(k.clone(), 0 * STOR)
					}
				})
				.collect(),
		},

		validator_set: ValidatorSetConfig {
			initial_validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},

		democracy: DemocracyConfig::default(),
 
		council: CouncilConfig::default(),
 
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
 
		// aura: Default::default(),
		aura: AuraConfig {

			authorities: vec![],
		},
		grandpa: GrandpaConfig {

			authorities: vec![],
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		treasury: Default::default(),
		transaction_payment: Default::default(),

		evm : Default::default(),

		session: SessionConfig {
			keys: initial_authorities
				.into_iter()
				.map(|(acc, aura, gran, im_online)| {	
					(
						acc.clone(), acc,
						session_keys(
							aura, gran, im_online,
						),
					
					)
				})
				.collect::<Vec<_>>(),
		},

		ethereum: Default::default(),
		base_fee: Default::default(),
		
	}
}

fn mainnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {

		treasury: Default::default(),
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},

		balances: BalancesConfig {

			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| {
					// if k == AccountId::from("0x90E79DAc498b35096d4d86CEa4f2c3681b40F5C7"). {
					if k == array_bytes::hex_n_into_unchecked(BALTATHAR) {
						(k.clone(), 1_755_000_000 * STOR)
					} else if k == array_bytes::hex_n_into_unchecked(CHARLETH) {
						(k.clone(), 66_000_000 * STOR)
					} else if k == array_bytes::hex_n_into_unchecked(DOROTHY) {
						(k.clone(), 194_999_000 * STOR)
					} else if k == array_bytes::hex_n_into_unchecked(ALITH) {
						(k.clone(), 1000 * STOR)
					} else {
						(k.clone(), 0 * STOR)
					}
				})
				.collect(),
		},

		validator_set: ValidatorSetConfig {
			initial_validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},

		aura: AuraConfig {
			authorities: vec![],
		},
		grandpa: GrandpaConfig {
			authorities: vec![],	
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		im_online: ImOnlineConfig { keys: vec![] },

		democracy: DemocracyConfig::default(),

		council: CouncilConfig::default(),

		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take(num_endowed_accounts  )
				.cloned()
				.collect(),
			phantom: Default::default(),
		},

		transaction_payment: Default::default(),
		evm : Default::default(),

		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(), 
						x.0.clone(), 
						session_keys(x.1.clone(), x.2.clone(), x.3.clone()))
				})
				.collect::<Vec<_>>(),
		},

		ethereum: Default::default(),
		base_fee: Default::default(),
	}
}
