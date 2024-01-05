use hex_literal::hex;
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
// use sp_runtime::key_types::IM_ONLINE;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_core::crypto::UncheckedInto;

use oslo_network_runtime::{
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

const ALITH: &str = "0x83451391e196556A66ebfCe472165d37E6575F5e";
const BALTATHAR: &str = "0xB78ef962F15Fb30d70fE7f5e00aA042869a2293A";
const CHARLETH: &str = "0x8ff34400aAb1Ee14Ee26BE799e107BDAAD88df8d";
const DOROTHY: &str = "0x8cBcD18e730eFD8c9C26e6791455CD100477DAEe";

pub fn public_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"OsloNetwork Mainnet",
		"public_live",
		ChainType::Live,
		move || {
			mainnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						hex!["bee4af052cb7b5d26a18565e9d3fe711d71dac78c80b99fb031a0d8a0f44e334"].unchecked_into(),
						hex!["153b1d1530fbc81e838fdd1295ab63afb95ef4c6e127e65966e7f1aa16d67cff"].unchecked_into(),
						hex!["70f95a75bfb4d4d13f0da0ba588bb68b1e90e408e0cd817e7401946db3359372"].unchecked_into(),
					),
					(
						array_bytes::hex_n_into_unchecked(BALTATHAR),
						hex!["ae9467654308063a5b18702c9f3643979cb0fc0874faf984f823b6a428cc761c"].unchecked_into(),
						hex!["7a6919d7bda5e843b76f9a81bf0661198207a985208446a478ad363722d03f87"].unchecked_into(),
						hex!["04b1cd6d6cecafe60a77e10e486bae799acd0fe4304dafaaac5363f3292da27b"].unchecked_into(),					
					),

					(	
						array_bytes::hex_n_into_unchecked(CHARLETH),
						hex!["a40276e3ecbeacc040ac4c6ecd175e3cb2c56ef08e098ca94b191b3d0450605e"].unchecked_into(),
						hex!["794d9d7557e84b135368c70e6d3cd0c3b9e467382e416e52dc693e1216990431"].unchecked_into(),
						hex!["4ef19ca699a442497419c85743dbd74294d2f615784bb0ea59ec89474ffbef44"].unchecked_into(),					),
					(
						array_bytes::hex_n_into_unchecked(DOROTHY),
						hex!["2ef8edfb4f34f4ab87bc07d3c08e9e49220fb4e3384f5f90c2368e0974e35111"].unchecked_into(),
						hex!["e581d5042a799186dd5e95896edc68470b5f90ea0f55e51ffc753fdb8f321106"].unchecked_into(),
						hex!["caa0f01977f32ea37fb0f056fabb5d10e6a80834dc9e6e614a3984a81cb08727"].unchecked_into(),					),
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
			"/dns/node1.oslocrypto.com/tcp/30333/p2p/12D3KooWC9ptbtXAGv2omPsckQ9wgvqjEtc72BfCZWXXbkVQZ1y2".parse().unwrap(),
			"/dns/node1.oslocrypto.com/tcp/30334/p2p/12D3KooWAHYbFdT3kv8GrLgDoBPERzez8C6sjGfQf8qJZEWVeP4f".parse().unwrap(),
			"/dns/node1.oslocrypto.com/tcp/30335/p2p/12D3KooWFdG1KySNaeksJv76vZauDk51zLTHHkxAmmGogGUEjpcu".parse().unwrap(),
			"/dns/node1.oslocrypto.com/tcp/30336/p2p/12D3KooWEsNNh1Ldsc8nDD9A3Z2Ww15cp3zUKGuSYwawftoKoL3s".parse().unwrap(),
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
	properties.insert("tokenSymbol".into(), "OSLO".into());
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
				) 

				],
				// Sudo account
				array_bytes::hex_n_into_unchecked(ALITH),
				// Pre-funded accounts
				vec![
					array_bytes::hex_n_into_unchecked(ALITH),
					array_bytes::hex_n_into_unchecked(BALTATHAR),
					array_bytes::hex_n_into_unchecked(CHARLETH),
					array_bytes::hex_n_into_unchecked(DOROTHY),				],
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

pub fn testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	
	Ok(ChainSpec::from_genesis(
		// Name
		"Oslo-Network Testnet",
		// ID
		"Oslo-Network_Testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						hex!["bee4af052cb7b5d26a18565e9d3fe711d71dac78c80b99fb031a0d8a0f44e334"].unchecked_into(),
						hex!["153b1d1530fbc81e838fdd1295ab63afb95ef4c6e127e65966e7f1aa16d67cff"].unchecked_into(),
						hex!["70f95a75bfb4d4d13f0da0ba588bb68b1e90e408e0cd817e7401946db3359372"].unchecked_into(),
					),
					(
						array_bytes::hex_n_into_unchecked(BALTATHAR),
						hex!["ae9467654308063a5b18702c9f3643979cb0fc0874faf984f823b6a428cc761c"].unchecked_into(),
						hex!["04b1cd6d6cecafe60a77e10e486bae799acd0fe4304dafaaac5363f3292da27b"].unchecked_into(),
						hex!["7a6919d7bda5e843b76f9a81bf0661198207a985208446a478ad363722d03f87"].unchecked_into(),					),
					(	
						array_bytes::hex_n_into_unchecked(CHARLETH),
						hex!["a40276e3ecbeacc040ac4c6ecd175e3cb2c56ef08e098ca94b191b3d0450605e"].unchecked_into(),
						hex!["794d9d7557e84b135368c70e6d3cd0c3b9e467382e416e52dc693e1216990431"].unchecked_into(),
						hex!["4ef19ca699a442497419c85743dbd74294d2f615784bb0ea59ec89474ffbef44"].unchecked_into(),					),
					(
						array_bytes::hex_n_into_unchecked(DOROTHY),
						hex!["2ef8edfb4f34f4ab87bc07d3c08e9e49220fb4e3384f5f90c2368e0974e35111"].unchecked_into(),
						hex!["e581d5042a799186dd5e95896edc68470b5f90ea0f55e51ffc753fdb8f321106"].unchecked_into(),
						hex!["caa0f01977f32ea37fb0f056fabb5d10e6a80834dc9e6e614a3984a81cb08727"].unchecked_into(),					),
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
			"/ip4/54.145.221.232/tcp/30333/p2p/12D3KooWC9ptbtXAGv2omPsckQ9wgvqjEtc72BfCZWXXbkVQZ1y2".parse().unwrap(),
			"/ip4/54.145.221.232/tcp/30334/p2p/12D3KooWAHYbFdT3kv8GrLgDoBPERzez8C6sjGfQf8qJZEWVeP4f".parse().unwrap(),
			"/ip4/54.145.221.232/tcp/30335/p2p/12D3KooWFdG1KySNaeksJv76vZauDk51zLTHHkxAmmGogGUEjpcu".parse().unwrap(),
			"/ip4/54.145.221.232/tcp/30336/p2p/12D3KooWEsNNh1Ldsc8nDD9A3Z2Ww15cp3zUKGuSYwawftoKoL3s".parse().unwrap(),
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

// /// Configure initial storage state for FRAME modules.
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
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else if k == array_bytes::hex_n_into_unchecked(CHARLETH) {
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else if k == array_bytes::hex_n_into_unchecked(DOROTHY) {
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else if k == array_bytes::hex_n_into_unchecked(ALITH) {
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else {
						(k.clone(), 0 * OSLO)
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
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else if k == array_bytes::hex_n_into_unchecked(CHARLETH) {
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else if k == array_bytes::hex_n_into_unchecked(DOROTHY) {
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else if k == array_bytes::hex_n_into_unchecked(ALITH) {
						(k.clone(), 2_500_000_000_000 * OSLO)
					} else {
						(k.clone(), 0 * OSLO)
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
