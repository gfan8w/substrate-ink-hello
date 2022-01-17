use codec::Encode;
/*use frame_support::log::{
    error,
    trace,
};*/

// 上传合约操作地址： https://paritytech.github.io/canvas-ui/#/

use pallet_contracts::chain_extension::{
    ChainExtension,
    Environment,
    Ext,
    InitState,
    RetVal,
    SysConfig,
    UncheckedFrom,
};
use sp_runtime::DispatchError;

use frame_support::traits::Randomness;

/// Contract extension for `FetchRandom`
pub struct FetchRandomExtension;

impl ChainExtension<super::Runtime> for FetchRandomExtension {
    fn call<E: Ext>(
        func_id: u32,
        env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId:
            UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        match func_id {
            1101 => {
                let mut env = env.buf_in_buf_out();
                let random_seed:[u8; 32] = super::RandomnessCollectiveFlip::random_seed().0.into();
                let random_slice = random_seed.encode();
                log::info!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&random_slice, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
            }

            _ => {
                log::error!("Called an unregistered `func_id`: {:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"))
            }
        }
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
