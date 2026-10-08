#![no_std]
mod storage;
mod treasury;
pub mod types;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};
use types::{Error, Group, MemberStatus, Proposal};

#[contract]
pub struct DuesTreasury;

#[contractimpl]
impl DuesTreasury {
    #[allow(clippy::too_many_arguments)]
    pub fn create_group(
        env: Env,
        admin: Address,
        token: Address,
        members: Vec<Address>,
        signers: Vec<Address>,
        threshold: u32,
        dues_amount: i128,
        period_seconds: u64,
    ) -> Result<u64, Error> {
        treasury::create_group(
            &env,
            admin,
            token,
            members,
            signers,
            threshold,
            dues_amount,
            period_seconds,
        )
    }
    pub fn pay_dues(env: Env, group_id: u64, member: Address) -> Result<u64, Error> {
        treasury::pay_dues(&env, group_id, member)
    }
    pub fn propose(
        env: Env,
        group_id: u64,
        signer: Address,
        to: Address,
        amount: i128,
        memo_hash: BytesN<32>,
    ) -> Result<u64, Error> {
        treasury::propose(&env, group_id, signer, to, amount, memo_hash)
    }
    pub fn approve(env: Env, proposal_id: u64, signer: Address) -> Result<(), Error> {
        treasury::approve(&env, proposal_id, signer)
    }
    pub fn execute(env: Env, proposal_id: u64) -> Result<(), Error> {
        treasury::execute(&env, proposal_id)
    }
    pub fn cancel(env: Env, proposal_id: u64, authorizers: Vec<Address>) -> Result<(), Error> {
        treasury::cancel(&env, proposal_id, authorizers)
    }
    pub fn get_group(env: Env, group_id: u64) -> Result<Group, Error> {
        storage::group(&env, group_id)
    }
    pub fn get_proposal(env: Env, proposal_id: u64) -> Result<Proposal, Error> {
        storage::proposal(&env, proposal_id)
    }
    pub fn member_status(env: Env, group_id: u64, member: Address) -> Result<MemberStatus, Error> {
        treasury::member_status(&env, group_id, member)
    }
}
#[cfg(test)]
mod error_paths;
#[cfg(test)]
mod test;
