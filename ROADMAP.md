# Roadmap

What is next for `duestreasury-contracts`, in order. Anything not listed as done is
**not implemented**.

## Status

- [x] Repository governance: AGENTS.md, CONTRIBUTING.md, ROADMAP.md, LICENSE,
      .gitignore, .gitattributes (2026-10-01).
- [ ] v0 contract from the project's playbook section.

## Next

- [ ] v0 contract from `STELLAR-BUILD-PLAYBOOK-v3.md` section 7
      (present in `~/Desktop/Drips/_reference/playbooks/`, confirmed
      2026-10-02). v0 scope from that section: a group collects dues from
      members into a treasury and spends only when enough designated
      signers approve. Entrypoints: `create_group` (fixed member and
      signer lists, `1 <= threshold <= signers.len()`), `pay_dues` (one
      payment per member per period), `propose`, `approve`, `execute`
      (anyone once threshold met), `cancel` (proposer or threshold),
      `get_group`, `member_status`, `get_proposal`. `memo_hash` keeps
      spending descriptions off-chain. Core invariant: balance equals dues
      received minus executed spends, tested after every step. Planned per
      the program stack: thin `lib.rs`; `types.rs` (error enum, stored
      types, events); `storage.rs`; `error_paths.rs` with one test per
      variant; `test.rs` lifecycle tests; `ERRORS.md` +
      `scripts/check-errors.mjs` and its tests; rust-toolchain pinned to
      `wasm32v1-none`; release profile with `overflow-checks = true`.
- [ ] Day 9 hardening: balance-invariant tests after every step, review
      checklist, SECURITY.md, TESTING.md, DEPLOYMENT_CHECKLIST.md.
- [ ] CI (`contract.yml`): fmt, clippy -D warnings, cargo test, node --test
      scripts/, check-errors, `stellar contract build` (CLI v28.1.0). Lands
      with the first code that can pass it.

## Deliberately unimplemented (from playbook section 7)

Listed there as the v0 boundary; each will get a draft issue when the v0
contract lands:

- Adding or removing members and signers by proposal.
- Proposal expiry.
- Per-period spending limits.
- Signer key rotation.
- Refund if a group dissolves.
- Audit-trail export.
- Property-based tests on the balance invariant.
- A security review checklist issue.

## Decisions needed from Tim

1. **Build standard — decided (2026-10-02).** v3 section 7 is the scope
   authority for what the contract does; v4 plus the schoolfees repos are
   the standard for how it is built (doc set, AGENTS.md, CI, checkers).
2. **Second reviewer.** Name the second human reviewer required before any funded test.

## Explicitly out of scope

Mainnet deployment. Anything the v0 design does not ask for.
