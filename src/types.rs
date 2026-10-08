use soroban_sdk::{contracterror, contractevent, contracttype, Address, BytesN, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    GroupNotFound = 1,
    ProposalNotFound = 2,
    InvalidMembers = 3,
    InvalidSigners = 4,
    InvalidThreshold = 5,
    InvalidAmount = 6,
    InvalidPeriod = 7,
    NotMember = 8,
    NotSigner = 9,
    AlreadyPaid = 10,
    AlreadyApproved = 11,
    ClosedProposal = 12,
    BelowThreshold = 13,
    InsufficientBalance = 14,
    Overflow = 15,
    InvalidRecipient = 16,
    InvalidCancellation = 17,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    pub id: u64,
    pub admin: Address,
    pub token: Address,
    pub members: Vec<Address>,
    pub signers: Vec<Address>,
    pub threshold: u32,
    pub dues_amount: i128,
    pub period_seconds: u64,
    pub started_at: u64,
    pub received: i128,
    pub spent: i128,
    pub balance: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Executed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u64,
    pub group_id: u64,
    pub proposer: Address,
    pub to: Address,
    pub amount: i128,
    pub memo_hash: BytesN<32>,
    pub approvals: Vec<Address>,
    pub status: ProposalStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberStatus {
    pub period: u64,
    pub paid: bool,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct TreasuryEvent {
    #[topic]
    pub group_id: u64,
    #[topic]
    pub action: soroban_sdk::Symbol,
    pub record_id: u64,
    pub actor: Address,
    pub amount: i128,
}
