use super::*;
use soroban_sdk::testutils::Events as _;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token, vec,
};

pub struct Fixture {
    pub env: Env,
    pub contract: Address,
    pub token: Address,
    pub admin: Address,
    pub members: Vec<Address>,
    pub signers: Vec<Address>,
}

#[test]
fn reading_group_and_paid_markers_renews_ttl() {
    use soroban_sdk::testutils::storage::{Instance as _, Persistent as _};
    let f = Fixture::new();
    let g = f.group();
    let m = f.members.get(0).unwrap();
    f.client().pay_dues(&g, &m);
    let maximum = f.env.storage().max_ttl();
    f.env
        .ledger()
        .set_sequence_number(f.env.ledger().sequence() + maximum / 2 + 1);
    let group_key = storage::DataKey::Group(g);
    let paid_key = storage::DataKey::Paid(g, m.clone(), 0);
    let before = f.env.as_contract(&f.contract, || {
        f.env.storage().persistent().get_ttl(&group_key)
    });
    f.client().get_group(&g);
    f.client().member_status(&g, &m);
    let after = f.env.as_contract(&f.contract, || {
        f.env.storage().persistent().get_ttl(&group_key)
    });
    let paid = f.env.as_contract(&f.contract, || {
        f.env.storage().persistent().get_ttl(&paid_key)
    });
    let instance = f
        .env
        .as_contract(&f.contract, || f.env.storage().instance().get_ttl());
    assert!(after > before);
    assert!(paid >= maximum - 1);
    assert!(instance >= maximum - 1);
}
#[test]
fn duplicate_members_and_signers_are_rejected() {
    let f = Fixture::new();
    let m = f.members.get(0).unwrap();
    let s = f.signers.get(0).unwrap();
    assert_eq!(
        f.client().try_create_group(
            &f.admin,
            &f.token,
            &vec![&f.env, m.clone(), m],
            &f.signers,
            &2,
            &100,
            &86_400
        ),
        Err(Ok(Error::InvalidMembers))
    );
    assert_eq!(
        f.client().try_create_group(
            &f.admin,
            &f.token,
            &f.members,
            &vec![&f.env, s.clone(), s],
            &2,
            &100,
            &86_400
        ),
        Err(Ok(Error::InvalidSigners))
    );
}
#[test]
fn upper_threshold_and_period_boundaries_are_enforced() {
    let f = Fixture::new();
    assert_eq!(
        f.client()
            .try_create_group(&f.admin, &f.token, &f.members, &f.signers, &4, &100, &86_400),
        Err(Ok(Error::InvalidThreshold))
    );
    assert_eq!(
        f.client().try_create_group(
            &f.admin,
            &f.token,
            &f.members,
            &f.signers,
            &2,
            &100,
            &(treasury::MAX_PERIOD_SECONDS + 1)
        ),
        Err(Ok(Error::InvalidPeriod))
    );
    let id = f.client().create_group(
        &f.admin,
        &f.token,
        &f.members,
        &f.signers,
        &3,
        &100,
        &treasury::MAX_PERIOD_SECONDS,
    );
    f.invariant(id, 0);
}

#[test]
fn duplicate_cancellation_signers_cannot_satisfy_threshold() {
    let f = Fixture::new();
    let p = f.proposal(f.group(), 10);
    let signer = f.signers.get(1).unwrap();
    assert_eq!(
        f.client()
            .try_cancel(&p, &vec![&f.env, signer.clone(), signer]),
        Err(Ok(Error::InvalidCancellation))
    );
    assert_eq!(
        f.client().get_proposal(&p).status,
        types::ProposalStatus::Pending
    );
}
#[test]
fn spend_approvals_do_not_authorize_cancellation() {
    let f = Fixture::new();
    let p = f.proposal(f.group(), 10);
    f.client().approve(&p, &f.signers.get(1).unwrap());
    f.client().approve(&p, &f.signers.get(2).unwrap());
    f.env.mock_auths(&[]);
    assert!(f
        .client()
        .try_cancel(
            &p,
            &vec![&f.env, f.signers.get(1).unwrap(), f.signers.get(2).unwrap()]
        )
        .is_err());
    assert_eq!(
        f.client().get_proposal(&p).status,
        types::ProposalStatus::Pending
    );
}
#[test]
fn executed_proposals_cannot_execute_approve_or_cancel_again() {
    let f = Fixture::new();
    let g = f.group();
    f.client().pay_dues(&g, &f.members.get(0).unwrap());
    let p = f.proposal(g, 100);
    f.client().approve(&p, &f.signers.get(0).unwrap());
    f.client().approve(&p, &f.signers.get(1).unwrap());
    f.client().execute(&p);
    assert_eq!(f.client().try_execute(&p), Err(Ok(Error::ClosedProposal)));
    assert_eq!(
        f.client().try_approve(&p, &f.signers.get(2).unwrap()),
        Err(Ok(Error::ClosedProposal))
    );
    assert_eq!(
        f.client()
            .try_cancel(&p, &vec![&f.env, f.signers.get(0).unwrap()]),
        Err(Ok(Error::ClosedProposal))
    );
    f.invariant(g, 0);
}
#[test]
fn lifecycle_publishes_each_documented_event() {
    let f = Fixture::new();
    let g = f.group();
    f.event("created");
    f.client().pay_dues(&g, &f.members.get(0).unwrap());
    f.event("dues");
    let p = f.proposal(g, 50);
    f.event("proposed");
    f.client().approve(&p, &f.signers.get(0).unwrap());
    f.event("approved");
    f.client().approve(&p, &f.signers.get(1).unwrap());
    f.event("approved");
    f.client().execute(&p);
    f.event("executed");
    let cancelled = f.proposal(g, 10);
    f.event("proposed");
    f.client()
        .cancel(&cancelled, &vec![&f.env, f.signers.get(0).unwrap()]);
    f.event("cancelled");
}
impl Fixture {
    fn event(&self, action: &str) {
        let own = self.env.events().all().filter_by_contract(&self.contract);
        assert_eq!(own.events().len(), 1);
        let soroban_sdk::xdr::ContractEventBody::V0(body) = &own.events()[0].body;
        let topics = &body.topics;
        assert_eq!(topics.len(), 3);
        assert_eq!(
            topics[0],
            soroban_sdk::xdr::ScVal::Symbol(soroban_sdk::xdr::ScSymbol(
                "treasury_event".try_into().unwrap()
            ))
        );
        assert!(topics.contains(&soroban_sdk::xdr::ScVal::Symbol(
            soroban_sdk::xdr::ScSymbol(action.try_into().unwrap())
        )));
    }
    pub fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract = env.register(DuesTreasury, ());
        let admin = Address::generate(&env);
        let token = env
            .register_stellar_asset_contract_v2(admin.clone())
            .address();
        let members = vec![&env, Address::generate(&env), Address::generate(&env)];
        let signers = vec![
            &env,
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];
        let mint = token::StellarAssetClient::new(&env, &token);
        for member in members.iter() {
            mint.mint(&member, &1_000);
        }
        Self {
            env,
            contract,
            token,
            admin,
            members,
            signers,
        }
    }
    pub fn client(&self) -> DuesTreasuryClient<'_> {
        DuesTreasuryClient::new(&self.env, &self.contract)
    }
    pub fn group(&self) -> u64 {
        self.client().create_group(
            &self.admin,
            &self.token,
            &self.members,
            &self.signers,
            &2,
            &100,
            &86_400,
        )
    }
    pub fn proposal(&self, group: u64, amount: i128) -> u64 {
        self.client().propose(
            &group,
            &self.signers.get(0).unwrap(),
            &Address::generate(&self.env),
            &amount,
            &BytesN::from_array(&self.env, &[7; 32]),
        )
    }
    pub fn invariant(&self, group: u64, expected: i128) {
        let value = self.client().get_group(&group);
        assert_eq!(value.balance, expected);
        assert_eq!(value.balance, value.received - value.spent);
    }
}
#[test]
fn threshold_lifecycle_preserves_accounting_after_every_step() {
    let f = Fixture::new();
    let group = f.group();
    f.invariant(group, 0);
    f.client().pay_dues(&group, &f.members.get(0).unwrap());
    f.invariant(group, 100);
    f.client().pay_dues(&group, &f.members.get(1).unwrap());
    f.invariant(group, 200);
    let proposal = f.proposal(group, 150);
    f.invariant(group, 200);
    f.client().approve(&proposal, &f.signers.get(0).unwrap());
    f.invariant(group, 200);
    f.client().approve(&proposal, &f.signers.get(1).unwrap());
    f.invariant(group, 200);
    f.client().execute(&proposal);
    f.invariant(group, 50);
    assert_eq!(
        token::TokenClient::new(&f.env, &f.token).balance(&f.contract),
        50
    );
    let p = f.client().get_proposal(&proposal);
    assert_eq!(p.status, types::ProposalStatus::Executed);
    assert_eq!(
        token::TokenClient::new(&f.env, &f.token).balance(&p.to),
        150
    );
}
#[test]
fn periods_allow_one_payment_each_without_resetting_accounting() {
    let f = Fixture::new();
    let g = f.group();
    let member = f.members.get(0).unwrap();
    assert!(!f.client().member_status(&g, &member).paid);
    assert_eq!(f.client().pay_dues(&g, &member), 0);
    assert!(f.client().member_status(&g, &member).paid);
    f.env.ledger().set_timestamp(86_400);
    assert!(!f.client().member_status(&g, &member).paid);
    assert_eq!(f.client().pay_dues(&g, &member), 1);
    f.invariant(g, 200);
}
#[test]
fn groups_share_token_custody_but_cannot_spend_each_others_balance() {
    let f = Fixture::new();
    let a = f.group();
    let b = f.group();
    f.client().pay_dues(&a, &f.members.get(0).unwrap());
    let p = f.proposal(b, 100);
    f.client().approve(&p, &f.signers.get(0).unwrap());
    f.client().approve(&p, &f.signers.get(1).unwrap());
    assert_eq!(
        f.client().try_execute(&p),
        Err(Ok(Error::InsufficientBalance))
    );
    f.invariant(a, 100);
    f.invariant(b, 0);
}
#[test]
fn proposer_and_explicit_threshold_can_cancel() {
    let f = Fixture::new();
    let g = f.group();
    let p = f.proposal(g, 10);
    f.client()
        .cancel(&p, &vec![&f.env, f.signers.get(0).unwrap()]);
    assert_eq!(
        f.client().get_proposal(&p).status,
        types::ProposalStatus::Cancelled
    );
    let p = f.proposal(g, 10);
    f.client().cancel(
        &p,
        &vec![&f.env, f.signers.get(1).unwrap(), f.signers.get(2).unwrap()],
    );
    f.invariant(g, 0);
}
#[test]
fn failed_token_transfer_rolls_back_dues_and_accounting() {
    let f = Fixture::new();
    let g = f.group();
    let m = f.members.get(0).unwrap();
    token::TokenClient::new(&f.env, &f.token).transfer(&m, Address::generate(&f.env), &1_000);
    assert!(f.client().try_pay_dues(&g, &m).is_err());
    assert!(!f.client().member_status(&g, &m).paid);
    f.invariant(g, 0);
}
#[test]
fn failed_execution_rolls_back_proposal_and_group() {
    let f = Fixture::new();
    let g = f.group();
    f.client().pay_dues(&g, &f.members.get(0).unwrap());
    let p = f.proposal(g, 100);
    f.client().approve(&p, &f.signers.get(0).unwrap());
    f.client().approve(&p, &f.signers.get(1).unwrap());
    token::TokenClient::new(&f.env, &f.token).transfer(
        &f.contract,
        Address::generate(&f.env),
        &100,
    );
    assert!(f.client().try_execute(&p).is_err());
    assert_eq!(
        f.client().get_proposal(&p).status,
        types::ProposalStatus::Pending
    );
    f.invariant(g, 100);
}
#[test]
fn missing_authorization_rejects_admin_member_signer_and_cancel() {
    let f = Fixture::new();
    let g = f.group();
    let p = f.proposal(g, 10);
    f.env.mock_auths(&[]);
    assert!(f
        .client()
        .try_create_group(&f.admin, &f.token, &f.members, &f.signers, &2, &100, &86_400)
        .is_err());
    assert!(f
        .client()
        .try_pay_dues(&g, &f.members.get(0).unwrap())
        .is_err());
    assert!(f
        .client()
        .try_approve(&p, &f.signers.get(1).unwrap())
        .is_err());
    assert!(f
        .client()
        .try_cancel(&p, &vec![&f.env, f.signers.get(0).unwrap()])
        .is_err());
}
