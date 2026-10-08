use super::*;
use crate::storage::DataKey;
use crate::test::Fixture;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    vec,
};
#[test]
fn error_path_group_not_found() {
    let f = Fixture::new();
    assert_eq!(f.client().try_get_group(&1), Err(Ok(Error::GroupNotFound)));
}
#[test]
fn error_path_proposal_not_found() {
    let f = Fixture::new();
    assert_eq!(
        f.client().try_get_proposal(&1),
        Err(Ok(Error::ProposalNotFound))
    );
}
#[test]
fn error_path_invalid_members() {
    let f = Fixture::new();
    assert_eq!(
        f.client().try_create_group(
            &f.admin,
            &f.token,
            &vec![&f.env],
            &f.signers,
            &2,
            &100,
            &86_400
        ),
        Err(Ok(Error::InvalidMembers))
    );
}
#[test]
fn error_path_invalid_signers() {
    let f = Fixture::new();
    assert_eq!(
        f.client().try_create_group(
            &f.admin,
            &f.token,
            &f.members,
            &vec![&f.env],
            &2,
            &100,
            &86_400
        ),
        Err(Ok(Error::InvalidSigners))
    );
}
#[test]
fn error_path_invalid_threshold() {
    let f = Fixture::new();
    assert_eq!(
        f.client()
            .try_create_group(&f.admin, &f.token, &f.members, &f.signers, &0, &100, &86_400),
        Err(Ok(Error::InvalidThreshold))
    );
}
#[test]
fn error_path_invalid_amount() {
    let f = Fixture::new();
    assert_eq!(
        f.client()
            .try_create_group(&f.admin, &f.token, &f.members, &f.signers, &2, &0, &86_400),
        Err(Ok(Error::InvalidAmount))
    );
}
#[test]
fn error_path_invalid_period() {
    let f = Fixture::new();
    assert_eq!(
        f.client()
            .try_create_group(&f.admin, &f.token, &f.members, &f.signers, &2, &100, &0),
        Err(Ok(Error::InvalidPeriod))
    );
}
#[test]
fn error_path_not_member() {
    let f = Fixture::new();
    let g = f.group();
    assert_eq!(
        f.client().try_pay_dues(&g, &Address::generate(&f.env)),
        Err(Ok(Error::NotMember))
    );
}
#[test]
fn error_path_not_signer() {
    let f = Fixture::new();
    let g = f.group();
    assert_eq!(
        f.client().try_propose(
            &g,
            &f.members.get(0).unwrap(),
            &Address::generate(&f.env),
            &10,
            &BytesN::from_array(&f.env, &[0; 32])
        ),
        Err(Ok(Error::NotSigner))
    );
}
#[test]
fn error_path_already_paid() {
    let f = Fixture::new();
    let g = f.group();
    let m = f.members.get(0).unwrap();
    f.client().pay_dues(&g, &m);
    assert_eq!(f.client().try_pay_dues(&g, &m), Err(Ok(Error::AlreadyPaid)));
}
#[test]
fn error_path_already_approved() {
    let f = Fixture::new();
    let p = f.proposal(f.group(), 10);
    let s = f.signers.get(0).unwrap();
    f.client().approve(&p, &s);
    assert_eq!(
        f.client().try_approve(&p, &s),
        Err(Ok(Error::AlreadyApproved))
    );
}
#[test]
fn error_path_closed_proposal() {
    let f = Fixture::new();
    let p = f.proposal(f.group(), 10);
    f.client()
        .cancel(&p, &vec![&f.env, f.signers.get(0).unwrap()]);
    assert_eq!(f.client().try_execute(&p), Err(Ok(Error::ClosedProposal)));
}
#[test]
fn error_path_below_threshold() {
    let f = Fixture::new();
    let p = f.proposal(f.group(), 10);
    assert_eq!(f.client().try_execute(&p), Err(Ok(Error::BelowThreshold)));
}
#[test]
fn error_path_insufficient_balance() {
    let f = Fixture::new();
    let p = f.proposal(f.group(), 10);
    f.client().approve(&p, &f.signers.get(0).unwrap());
    f.client().approve(&p, &f.signers.get(1).unwrap());
    assert_eq!(
        f.client().try_execute(&p),
        Err(Ok(Error::InsufficientBalance))
    );
}
#[test]
fn error_path_overflow() {
    let f = Fixture::new();
    f.env.as_contract(&f.contract, || {
        f.env
            .storage()
            .instance()
            .set(&DataKey::GroupCounter, &u64::MAX)
    });
    assert_eq!(
        f.client()
            .try_create_group(&f.admin, &f.token, &f.members, &f.signers, &2, &100, &86_400),
        Err(Ok(Error::Overflow))
    );
}
#[test]
fn error_path_invalid_recipient() {
    let f = Fixture::new();
    assert_eq!(
        f.client().try_propose(
            &f.group(),
            &f.signers.get(0).unwrap(),
            &f.contract,
            &10,
            &BytesN::from_array(&f.env, &[0; 32])
        ),
        Err(Ok(Error::InvalidRecipient))
    );
}
#[test]
fn error_path_invalid_cancellation() {
    let f = Fixture::new();
    let p = f.proposal(f.group(), 10);
    assert_eq!(
        f.client()
            .try_cancel(&p, &vec![&f.env, f.signers.get(1).unwrap()]),
        Err(Ok(Error::InvalidCancellation))
    );
}
#[test]
fn timestamp_regression_rejects_period() {
    let f = Fixture::new();
    f.env.ledger().set_timestamp(100);
    let g = f.group();
    f.env.ledger().set_timestamp(99);
    assert_eq!(
        f.client().try_member_status(&g, &f.members.get(0).unwrap()),
        Err(Ok(Error::InvalidPeriod))
    );
}
