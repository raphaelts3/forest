// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::Error;
use cid::Cid;
use forest_chain::MINIMUM_BASE_FEE;
use forest_message::{Message as MessageTrait, SignedMessage};
use fvm_ipld_encoding::Cbor;
use fvm_shared::bigint::{BigInt, Integer};
use fvm_shared::crypto::signature::Signature;
use fvm_shared::message::Message;
use lru::LruCache;
use num_rational::BigRational;
use num_traits::ToPrimitive;

pub(crate) fn get_base_fee_lower_bound(base_fee: &BigInt, factor: i64) -> BigInt {
    let base_fee_lower_bound = base_fee.div_floor(&BigInt::from(factor));
    if base_fee_lower_bound < *MINIMUM_BASE_FEE {
        return MINIMUM_BASE_FEE.clone();
    }
    base_fee_lower_bound
}

/// Gets the gas reward for the given message.
pub(crate) fn get_gas_reward(msg: &SignedMessage, base_fee: &BigInt) -> BigInt {
    let mut max_prem = msg.gas_fee_cap() - base_fee;
    if &max_prem < msg.gas_premium() {
        max_prem = msg.gas_premium().clone();
    }
    max_prem * msg.gas_limit()
}

pub(crate) fn get_gas_perf(gas_reward: &BigInt, gas_limit: i64) -> f64 {
    let a = BigRational::new(gas_reward * fvm_shared::BLOCK_GAS_LIMIT, gas_limit.into());
    a.to_f64().unwrap()
}

/// Attempt to get a signed message that corresponds to an unsigned message in `bls_sig_cache`.
pub(crate) async fn recover_sig(
    bls_sig_cache: &mut LruCache<Cid, Signature>,
    msg: Message,
) -> Result<SignedMessage, Error> {
    let val = bls_sig_cache
        .get(&msg.cid()?)
        .ok_or_else(|| Error::Other("Could not recover sig".to_owned()))?;
    let smsg = SignedMessage::new_from_parts(msg, val.clone()).map_err(Error::Other)?;
    Ok(smsg)
}
