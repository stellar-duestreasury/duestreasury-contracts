use crate::types::{Error, Group, Proposal};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    GroupCounter,
    ProposalCounter,
    Group(u64),
    Proposal(u64),
    Paid(u64, Address, u64),
}

// Groups and proposals do not expire. Extend active records toward the network
// maximum; archived entries still require restoration, never reset accounting.
pub fn touch(env: &Env, key: &DataKey) {
    let target = env.storage().max_ttl();
    env.storage()
        .persistent()
        .extend_ttl(key, target / 2, target);
    env.storage().instance().extend_ttl(target / 2, target);
}
pub fn next(env: &Env, key: &DataKey) -> Result<u64, Error> {
    let value: u64 = env.storage().instance().get(key).unwrap_or(0);
    let id = value.checked_add(1).ok_or(Error::Overflow)?;
    env.storage().instance().set(key, &id);
    let target = env.storage().max_ttl();
    env.storage().instance().extend_ttl(target / 2, target);
    Ok(id)
}
pub fn group(env: &Env, id: u64) -> Result<Group, Error> {
    let key = DataKey::Group(id);
    let record = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(Error::GroupNotFound)?;
    touch(env, &key);
    Ok(record)
}
pub fn proposal(env: &Env, id: u64) -> Result<Proposal, Error> {
    let key = DataKey::Proposal(id);
    let record = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(Error::ProposalNotFound)?;
    touch(env, &key);
    Ok(record)
}
pub fn save_group(env: &Env, record: &Group) {
    let key = DataKey::Group(record.id);
    env.storage().persistent().set(&key, record);
    touch(env, &key);
}
pub fn save_proposal(env: &Env, record: &Proposal) {
    let key = DataKey::Proposal(record.id);
    env.storage().persistent().set(&key, record);
    touch(env, &key);
}
