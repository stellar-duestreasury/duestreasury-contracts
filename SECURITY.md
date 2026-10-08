# Security and custody

Testnet only. This contract is not independently reviewed and must not hold real funds. Deployment remains blocked; a second human reviewer is required before any funded test. Report a vulnerability privately to the maintainer through GitHub contact details; do not publish exploit instructions or secrets.

Fixed lists are bounded to 100 members and 20 signers. Every member payment and signer action requires Soroban address authorization. Execution is permissionless only after explicit threshold approvals. Cancellation consent is separate from spending consent. Group accounting cannot draw on another group, even when the token is shared. Effects precede token transfers and failed invocations roll back. Loss of signer keys can permanently block spending. Active records renew toward the network maximum TTL; archival requires explicit restoration, not reinitialization. Unused groups and proposals can archive.

The selected token contract must implement the standard transfer interface faithfully. There is no token allowlist or custom-token audit. Issuer freezes or clawbacks can make recorded funds unspendable, and direct transfers are not allocated to a group. v0 has no reconciliation or recovery flow for these conditions; the custody reviewer must assess token issuer controls before any funded test.
