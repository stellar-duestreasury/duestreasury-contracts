use crate::{
    storage::{self, DataKey},
    types::{Error, Group, MemberStatus, Proposal, ProposalStatus, TreasuryEvent},
};
use soroban_sdk::{symbol_short, token::TokenClient, Address, BytesN, Env, Symbol, Vec};
pub const MAX_MEMBERS: u32 = 100;
pub const MAX_SIGNERS: u32 = 20;
pub const MAX_PERIOD_SECONDS: u64 = 366 * 86400;

fn unique(values: &Vec<Address>, cap: u32) -> bool {
    if values.is_empty() || values.len() > cap {
        return false;
    }
    for (index, value) in values.iter().enumerate() {
        if values.iter().skip(index + 1).any(|other| other == value) {
            return false;
        }
    }
    true
}
fn emit(env: &Env, group_id: u64, action: Symbol, record_id: u64, actor: Address, amount: i128) {
    TreasuryEvent {
        group_id,
        action,
        record_id,
        actor,
        amount,
    }
    .publish(env);
}
fn period(env: &Env, group: &Group) -> Result<u64, Error> {
    env.ledger()
        .timestamp()
        .checked_sub(group.started_at)
        .ok_or(Error::InvalidPeriod)
        .map(|elapsed| elapsed / group.period_seconds)
}
fn pending(proposal: &Proposal) -> Result<(), Error> {
    if proposal.status != ProposalStatus::Pending {
        return Err(Error::ClosedProposal);
    }
    Ok(())
}
#[allow(clippy::too_many_arguments)]
pub fn create_group(
    env: &Env,
    admin: Address,
    token: Address,
    members: Vec<Address>,
    signers: Vec<Address>,
    threshold: u32,
    dues_amount: i128,
    period_seconds: u64,
) -> Result<u64, Error> {
    admin.require_auth();
    if !unique(&members, MAX_MEMBERS) {
        return Err(Error::InvalidMembers);
    }
    if !unique(&signers, MAX_SIGNERS) {
        return Err(Error::InvalidSigners);
    }
    if threshold == 0 || threshold > signers.len() {
        return Err(Error::InvalidThreshold);
    }
    if dues_amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    if period_seconds == 0 || period_seconds > MAX_PERIOD_SECONDS {
        return Err(Error::InvalidPeriod);
    }
    let id = storage::next(env, &DataKey::GroupCounter)?;
    storage::save_group(
        env,
        &Group {
            id,
            admin: admin.clone(),
            token,
            members,
            signers,
            threshold,
            dues_amount,
            period_seconds,
            started_at: env.ledger().timestamp(),
            received: 0,
            spent: 0,
            balance: 0,
        },
    );
    emit(env, id, symbol_short!("created"), id, admin, 0);
    Ok(id)
}
pub fn member_status(env: &Env, group_id: u64, member: Address) -> Result<MemberStatus, Error> {
    let group = storage::group(env, group_id)?;
    if !group.members.contains(&member) {
        return Err(Error::NotMember);
    }
    let period = period(env, &group)?;
    let key = DataKey::Paid(group_id, member, period);
    let paid = env.storage().persistent().has(&key);
    if paid {
        storage::touch(env, &key);
    }
    Ok(MemberStatus { period, paid })
}
pub fn pay_dues(env: &Env, group_id: u64, member: Address) -> Result<u64, Error> {
    member.require_auth();
    let status = member_status(env, group_id, member.clone())?;
    if status.paid {
        return Err(Error::AlreadyPaid);
    }
    let mut group = storage::group(env, group_id)?;
    group.received = group
        .received
        .checked_add(group.dues_amount)
        .ok_or(Error::Overflow)?;
    group.balance = group
        .balance
        .checked_add(group.dues_amount)
        .ok_or(Error::Overflow)?;
    // Effects precede external transfer; a failed host invocation rolls them back.
    let key = DataKey::Paid(group_id, member.clone(), status.period);
    env.storage().persistent().set(&key, &true);
    storage::touch(env, &key);
    storage::save_group(env, &group);
    TokenClient::new(env, &group.token).transfer(
        &member,
        env.current_contract_address(),
        &group.dues_amount,
    );
    emit(
        env,
        group_id,
        symbol_short!("dues"),
        status.period,
        member,
        group.dues_amount,
    );
    Ok(status.period)
}
pub fn propose(
    env: &Env,
    group_id: u64,
    signer: Address,
    to: Address,
    amount: i128,
    memo_hash: BytesN<32>,
) -> Result<u64, Error> {
    signer.require_auth();
    let group = storage::group(env, group_id)?;
    if !group.signers.contains(&signer) {
        return Err(Error::NotSigner);
    }
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    if to == env.current_contract_address() {
        return Err(Error::InvalidRecipient);
    }
    let id = storage::next(env, &DataKey::ProposalCounter)?;
    storage::save_proposal(
        env,
        &Proposal {
            id,
            group_id,
            proposer: signer.clone(),
            to,
            amount,
            memo_hash,
            approvals: Vec::new(env),
            status: ProposalStatus::Pending,
        },
    );
    emit(env, group_id, symbol_short!("proposed"), id, signer, amount);
    Ok(id)
}
pub fn approve(env: &Env, id: u64, signer: Address) -> Result<(), Error> {
    signer.require_auth();
    let mut proposal = storage::proposal(env, id)?;
    pending(&proposal)?;
    let group = storage::group(env, proposal.group_id)?;
    if !group.signers.contains(&signer) {
        return Err(Error::NotSigner);
    }
    if proposal.approvals.contains(&signer) {
        return Err(Error::AlreadyApproved);
    }
    proposal.approvals.push_back(signer.clone());
    storage::save_proposal(env, &proposal);
    emit(
        env,
        group.id,
        symbol_short!("approved"),
        id,
        signer,
        proposal.amount,
    );
    Ok(())
}
pub fn execute(env: &Env, id: u64) -> Result<(), Error> {
    let mut proposal = storage::proposal(env, id)?;
    pending(&proposal)?;
    let mut group = storage::group(env, proposal.group_id)?;
    if proposal.approvals.len() < group.threshold {
        return Err(Error::BelowThreshold);
    }
    if proposal.amount > group.balance {
        return Err(Error::InsufficientBalance);
    }
    group.balance = group
        .balance
        .checked_sub(proposal.amount)
        .ok_or(Error::Overflow)?;
    group.spent = group
        .spent
        .checked_add(proposal.amount)
        .ok_or(Error::Overflow)?;
    proposal.status = ProposalStatus::Executed;
    storage::save_group(env, &group);
    storage::save_proposal(env, &proposal);
    TokenClient::new(env, &group.token).transfer(
        &env.current_contract_address(),
        &proposal.to,
        &proposal.amount,
    );
    emit(
        env,
        group.id,
        symbol_short!("executed"),
        id,
        proposal.to,
        proposal.amount,
    );
    Ok(())
}
pub fn cancel(env: &Env, id: u64, authorizers: Vec<Address>) -> Result<(), Error> {
    let mut proposal = storage::proposal(env, id)?;
    pending(&proposal)?;
    let group = storage::group(env, proposal.group_id)?;
    if !unique(&authorizers, MAX_SIGNERS) {
        return Err(Error::InvalidCancellation);
    }
    let by_proposer = authorizers.len() == 1 && authorizers.contains(&proposal.proposer);
    if !by_proposer
        && (authorizers.len() < group.threshold
            || authorizers
                .iter()
                .any(|signer| !group.signers.contains(&signer)))
    {
        return Err(Error::InvalidCancellation);
    }
    for signer in authorizers.iter() {
        signer.require_auth();
    }
    proposal.status = ProposalStatus::Cancelled;
    storage::save_proposal(env, &proposal);
    emit(
        env,
        group.id,
        symbol_short!("cancelled"),
        id,
        proposal.proposer,
        proposal.amount,
    );
    Ok(())
}
