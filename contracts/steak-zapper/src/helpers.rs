use astroport::asset::{Asset, AssetInfo};
use cosmwasm_std::{Addr, Coin, Event, StdError, StdResult, Uint128};

/// Determine whether an event contains a specific key-value pair
pub(crate) fn event_contains_attr(event: &Event, key: &str, value: &str) -> bool {
    event.attributes.iter().any(|attr| attr.key == key && attr.value == value)
}

/// Create a new `Asset` instance of a CW20 token
pub(crate) fn new_cw20(contract_addr: Addr, amount: Uint128) -> Asset {
    Asset {
        info: AssetInfo::Token { contract_addr },
        amount,
    }
}

/// Create a new `Asset` instance of a native token based on funds received
pub(crate) fn new_native_from_funds(funds: &[Coin]) -> StdResult<Asset> {
    if funds.len() != 1 {
        return Err(StdError::generic_err(
            format!("must deposit exactly one coin; received {}", funds.len())
        ));
    }

    let fund = &funds[0];

    Ok(Asset {
        info: AssetInfo::NativeToken { denom: fund.denom.clone() },
        amount: fund.amount,
    })
}

/// Create a string representation of an asset. Here we use the `cw_asset` format, which is more
/// readable and easier to parse than the SDK format
///
/// More on `cw_asset`: https://github.com/mars-protocol/cw-asset
pub(crate) fn stringify_asset(asset: &Asset) -> String {
    match &asset.info {
        AssetInfo::NativeToken { denom } => format!("native:{}:{}", denom, asset.amount),
        AssetInfo::Token { contract_addr } => format!("cw20:{}:{}", contract_addr, asset.amount),
    }
}
