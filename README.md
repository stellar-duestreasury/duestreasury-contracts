# DuesTreasury

Implemented local v0 Soroban treasury: fixed members and signers, one dues payment per period, explicit proposal approvals, permissionless threshold execution, and proposer or threshold cancellation. **Testnet only; no real money. Not independently reviewed. Deployment is blocked pending a second human custody review. No pilot has happened.**

Contract entrypoints are in [src/lib.rs](src/lib.rs); caps are 100 members, 20 signers, periods up to 366 days. Tokens and amounts use atomic units; group balances equal received dues minus executed spending. [`cancel`](src/treasury.rs) takes explicit authorizer addresses, because spend approvals are not cancellation consent. Current period status does not calculate historical arrears.

See [TESTING.md](TESTING.md), [SECURITY.md](SECURITY.md), [ERRORS.md](ERRORS.md), [events](docs/events.md), [reuse decision](docs/decisions/0001-build-vs-reuse.md), and [deployment gate](DEPLOYMENT_CHECKLIST.md). Architecture lives in the [book repository](https://github.com/stellar-duestreasury/duestreasury-docs). App and contract have not been exercised together against a deployed instance.
