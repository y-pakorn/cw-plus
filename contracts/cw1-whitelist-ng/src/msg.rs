// This whole file except `AdminListResponse` shall be generated form contract traits and
// instantiate signature

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
};

use crate::error::ContractError;
use crate::interfaces::*;
use crate::state::Cw1WhitelistContract;

pub use crate::contract::msg::InstantiateMsg;

pub use crate::interfaces::{cw1_msg, whitelist};

/*
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", untagged)]
enum ExecMsg<T = Empty> {
    Cw1(Cw1ExecMsg<T>),
    Whitelist(WhitelistExecMsg),
}

impl<T> ExecMsg<T> {
    pub fn dispatch<Contract>(
        self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: &Contract,
    ) -> Result<Response<T>, <Contract as Cw1<T>>::Error>
    where
        Contract: Cw1<T> + Whitelist<T, Error = <Contract as Cw1<T>>::Error>,
    {
        use ExecMsg::*;

        match self {
            Cw1(msg) => msg.dispatch(deps, env, info, contract),
            Whitelist(msg) => msg.dispatch(deps, env, info, contract),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", untagged)]
enum QueryMsg<T = Empty> {
    Cw1(Cw1QueryMsg<T>),
    Whitelist(WhitelistQueryMsg),
}

impl<T> QueryMsg<T> {
    pub fn dispatch<Contract>(
        self,
        deps: Deps,
        env: Env,
        contract: &Contract,
    ) -> Result<Binary, <Contract as Cw1<T>>::Error>
    where
        Contract: Cw1<T> + Whitelist<T, Error = <Contract as Cw1<T>>::Error>,
        <Contract as Cw1<T>>::Error: From<StdError>,
    {
        use QueryMsg::*;

        match self {
            Cw1(msg) => msg.dispatch(deps, env, contract),
            Whitelist(msg) => msg.dispatch(deps, env, contract),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw1ExecMsg<T = Empty> {
    /// Execute requests the contract to re-dispatch all these messages with the
    /// contract's address as sender. Every implementation has it's own logic to
    /// determine in
    Execute { msgs: Vec<CosmosMsg<T>> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WhitelistExecMsg {
    /// Freeze will make a mutable contract immutable, must be called by an admin
    Freeze {},
    /// UpdateAdmins will change the admin set of the contract, must be called by an existing admin,
    /// and only works if the contract is mutable
    UpdateAdmins { admins: Vec<String> },
}

impl<T> Cw1ExecMsg<T> {
    pub fn dispatch<Contract>(
        self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: &Contract,
    ) -> Result<Response<T>, Contract::Error>
    where
        Contract: Cw1<T>,
    {
        use Cw1ExecMsg::*;

        match self {
            Execute { msgs } => contract.execute(deps, env, info, msgs),
        }
    }
}

impl WhitelistExecMsg {
    pub fn dispatch<T, Contract>(
        self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: &Contract,
    ) -> Result<Response<T>, Contract::Error>
    where
        Contract: Whitelist<T>,
    {
        use WhitelistExecMsg::*;

        match self {
            Freeze {} => contract.freeze(deps, env, info),
            UpdateAdmins { admins } => contract.update_admins(deps, env, info, admins),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw1QueryMsg<T = Empty> {
    /// Checks permissions of the caller on this proxy.
    /// If CanExecute returns true then a call to `Execute` with the same message,
    /// before any further state changes, should also succeed.
    CanExecute { sender: String, msg: CosmosMsg<T> },
}

impl<T> Cw1QueryMsg<T> {
    pub fn dispatch<Contract>(
        self,
        deps: Deps,
        env: Env,
        contract: &Contract,
    ) -> Result<Binary, Contract::Error>
    where
        Contract: Cw1<T>,
        Contract::Error: From<StdError>,
    {
        use Cw1QueryMsg::*;

        match self {
            CanExecute { sender, msg } => to_binary(&contract.can_execute(deps, env, sender, msg)?),
        }
        .map_err(Contract::Error::from)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WhitelistQueryMsg {
    /// Shows all admins and whether or not it is mutable
    AdminList {},
}

impl WhitelistQueryMsg {
    pub fn dispatch<T, Contract>(
        self,
        deps: Deps,
        env: Env,
        contract: &Contract,
    ) -> Result<Binary, Contract::Error>
    where
        Contract: Whitelist<T>,
        Contract::Error: From<StdError>,
    {
        use WhitelistQueryMsg::*;

        match self {
            AdminList {} => to_binary(&contract.admin_list(deps, env)?),
        }
        .map_err(Contract::Error::from)
    }
}*/

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AdminListResponse {
    pub admins: Vec<String>,
    pub mutable: bool,
}
