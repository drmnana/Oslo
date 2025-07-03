//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;
use std::collections::BTreeMap;
use jsonrpsee::RpcModule;
use oslo_network_runtime::{AccountId, Balance, Index};
use sc_client_api::{backend::{Backend, StorageProvider, StateBackend}, client::BlockchainEvents, AuxStore, UsageProvider};
use sc_network_sync::SyncingService;
use sc_network::service::traits::NetworkService;
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use sp_consensus_aura::{AuraApi, sr25519::AuthorityId as AuraId};
use sp_inherents::CreateInherentDataProviders;
use sp_api::{ProvideRuntimeApi, CallApiAt};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use fc_rpc::{EthBlockDataCacheTask, StorageOverrideHandler, pending::AuraConsensusDataProvider, EthApiServer};
use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit};
use fp_rpc::{ConvertTransaction, ConvertTransactionRuntimeApi, EthereumRuntimeRPCApi};
use sp_core::H256;
pub use sc_rpc_api::DenyUnsafe;
use sp_runtime::traits::{BlakeTwo256, Block as BlockT};

/// EVM overrides
pub fn overrides_handle<B, C, BE>(client: Arc<C>) -> Arc<StorageOverrideHandler<B, C, BE>>
	where
		C: ProvideRuntimeApi<B> + StorageProvider<B, BE> + AuxStore,
		C: HeaderBackend<B> + HeaderMetadata<B, Error=BlockChainError>,
		C: Send + Sync + 'static,
		C::Api: sp_api::ApiExt<B> + fp_rpc::EthereumRuntimeRPCApi<B> + fp_rpc::ConvertTransactionRuntimeApi<B>,
		BE: Backend<B> + 'static,
		BE::State: StateBackend<BlakeTwo256>,
		B: BlockT
{

	Arc::new(StorageOverrideHandler::new(client.clone()))
}


/// Full client dependencies.
/// Extra dependencies for Ethereum compatibility.
pub struct FullDeps<B: BlockT, C, P, A: ChainApi, CT, CIDP> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// Ethereum transaction converter.
	pub converter: Option<CT>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// Network service
	pub network: Arc<dyn NetworkService>,
	/// Chain syncing service
	pub sync: Arc<SyncingService<B>>,
	/// Frontier Backend.
	pub frontier_backend: Arc<dyn fc_api::Backend<B>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<B>>,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Maximum fee history cache size.
	pub fee_history_cache_limit: FeeHistoryCacheLimit,
	/// Maximum allowed gas limit will be ` block.gas_limit * execute_gas_limit_multiplier` when
	/// using eth_call/eth_estimateGas.
	pub execute_gas_limit_multiplier: u64,
	/// Mandated parent hashes for a given block hash.
	pub forced_parent_hashes: Option<BTreeMap<H256, H256>>,
	/// Something that can create the inherent data providers for pending state
	pub pending_create_inherent_data_providers: CIDP
}

///I have no clue
pub struct DefaultEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);

impl<B, C, BE> fc_rpc::EthConfig<B, C> for DefaultEthConfig<C, BE>
where
	B: BlockT,
	C: StorageProvider<B, BE> + Sync + Send + 'static,
	BE: Backend<B> + 'static
{
	///Why this is necessary
	type EstimateGasAdapter = ();
	///For Eth::new() to work
	type RuntimeStorageOverride = fc_rpc::frontier_backend_client::SystemAccountId20StorageOverride<B, C, BE>;
}
/// Instantiate all full RPC extensions.
pub fn create_full<B, C, BE, P, A, CT, CIDP>(
	deps: FullDeps<B, C, P, A, CT, CIDP>
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
	where
		B: BlockT,
		C: CallApiAt<B> + ProvideRuntimeApi<B> + AuxStore + UsageProvider<B>,
		C: BlockchainEvents<B> + StorageProvider<B, BE>,
		C: HeaderBackend<B> + HeaderMetadata<B, Error=BlockChainError> + 'static,
		C: Send + Sync + 'static,
		C::Api: substrate_frame_rpc_system::AccountNonceApi<B, AccountId, Index>,
		C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<B, Balance>,
		C::Api: AuraApi<B, AuraId> + BlockBuilder<B> + ConvertTransactionRuntimeApi<B> + EthereumRuntimeRPCApi<B>,
		BE: Backend<B> + 'static,
		BE::State: StateBackend<BlakeTwo256>,
		P: TransactionPool<Block=B> + 'static,
		A: ChainApi<Block=B> + 'static,
		CT: ConvertTransaction<<B as BlockT>::Extrinsic> + Send + Sync + 'static,
		CIDP: CreateInherentDataProviders<B, ()> + Send + 'static
{
	use fc_rpc::{Eth, Net, NetApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};
	let mut module = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		graph,
		converter,
		deny_unsafe,
		is_authority,
		network,
		sync,
		frontier_backend,
		block_data_cache,
		fee_history_cache,
		fee_history_cache_limit,
		execute_gas_limit_multiplier,
		forced_parent_hashes,
		pending_create_inherent_data_providers
	} = deps;
	// We won't use the override feature
	let overrides = Arc::new(StorageOverrideHandler::new(client.clone()));

	// Nor any signers
	let signers = Vec::new();
	
	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(Net::new(client.clone(), network.clone(), true).into_rpc())?;
	module.merge(Eth::<B, C, P, CT, BE, A, CIDP, DefaultEthConfig<C, BE>>::new(
		client.clone(),
		pool.clone(),
		graph.clone(),
		converter,
		sync.clone(),
		signers,
		overrides.clone(),
		frontier_backend.clone(),
		is_authority,
		block_data_cache.clone(),
		fee_history_cache.clone(),
		fee_history_cache_limit,
		execute_gas_limit_multiplier,
		forced_parent_hashes,
		pending_create_inherent_data_providers,
		Some(Box::new(AuraConsensusDataProvider::new(client.clone())))
	).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

	// Reasonable default caching inspired by the frontier template
	// let block_data_cache = Arc::new(EthBlockDataCacheTask::new(50, 50));

	Ok(module)
}
