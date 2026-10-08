# Roadmap

## Implemented

- Fixed-member, fixed-signer group creation and bounded threshold validation.
- Period dues, proposals, approvals, execution, explicit cancellation, and read methods.
- Checked accounting, authorization, token rollback tests and all error paths.
- Error checker, CI, custody/security documentation and blocked deployment script.

## Deliberately unimplemented

Membership changes, proposal expiry, period spending limits, key rotation, dissolution refunds, full audit export, property-based invariant tests, and independent custody review. These are recorded as draft issues in `docs/issue-drafts/`; no GitHub issues were opened.

## Decisions needed from Tim

Name a second human reviewer before any funded test. Deployment remains blocked. Mainnet is out of scope.
