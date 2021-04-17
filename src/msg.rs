use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // Address of the Mirror governance staking contract
    // Mainnet address: `terra17f7zu97865jmknk7p2glqvxzhduk78772ezac5`
    pub mirror_staking: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Implements CW20. Burns stMIR from the balance of `env.sender` and send appropriate
    /// amount of MIR back to sender
    Burn { amount: Uint128 },
    /// Implements CW20
    Transfer { recipient: String, amount: Uint128 },
    /// Implements CW20
    Send { contract: String, amount: Uint128, msg: Option<Binary> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Implements CW20
    Balance { address: String },
    /// Implements CW20
    TokenInfo {},
}
