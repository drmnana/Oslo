use pallet_evm::{Precompile, PrecompileFailure, PrecompileOutput, PrecompileHandle, IsPrecompileResult, PrecompileSet};
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};

pub struct SubstratePrecompiles<R>(PhantomData<R>);

impl<R> SubstratePrecompiles<R>
where
	R: pallet_evm::Config
{
	pub fn new() -> Self { Self(Default::default()) }
	pub fn used_addresses() -> sp_std::vec::Vec<H160> {
		sp_std::vec![1, 2, 3, 4, 5, 1024, 1025].into_iter().map(hash).collect()
	}
}
impl<R> PrecompileSet for SubstratePrecompiles<R>
where
	R: pallet_evm::Config
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<Result<PrecompileOutput, PrecompileFailure>> {
		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			// Non-Frontier specific nor Ethereum precompiles :
			a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
			_ => None
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer { is_precompile: Self::used_addresses().contains(&address), extra_cost: 0 }
	}
}

fn hash(a: u64) -> H160 { H160::from_low_u64_be(a) }