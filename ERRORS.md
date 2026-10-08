# DuesTreasury error codes

The user-facing column is the wording authority for the app. Host authorization and token errors do not have invented contract codes.

| Code | Variant | Raised by | Trigger | User-facing message | Next action |
|---:|---|---|---|---|---|
| 1 | `GroupNotFound` | Treasury operation | GroupNotFound guard | "This group could not be found." | Review the group and proposal, then retry only after correcting the input. |
| 2 | `ProposalNotFound` | Treasury operation | ProposalNotFound guard | "This proposal could not be found." | Review the group and proposal, then retry only after correcting the input. |
| 3 | `InvalidMembers` | Treasury operation | InvalidMembers guard | "Provide between 1 and 100 unique members." | Review the group and proposal, then retry only after correcting the input. |
| 4 | `InvalidSigners` | Treasury operation | InvalidSigners guard | "Provide between 1 and 20 unique signers." | Review the group and proposal, then retry only after correcting the input. |
| 5 | `InvalidThreshold` | Treasury operation | InvalidThreshold guard | "The approval threshold must be between 1 and the signer count." | Review the group and proposal, then retry only after correcting the input. |
| 6 | `InvalidAmount` | Treasury operation | InvalidAmount guard | "The amount must be greater than zero." | Review the group and proposal, then retry only after correcting the input. |
| 7 | `InvalidPeriod` | Treasury operation | InvalidPeriod guard | "The period must be between 1 second and 366 days." | Review the group and proposal, then retry only after correcting the input. |
| 8 | `NotMember` | Treasury operation | NotMember guard | "This address is not a member of the group." | Review the group and proposal, then retry only after correcting the input. |
| 9 | `NotSigner` | Treasury operation | NotSigner guard | "This address is not a signer for the group." | Review the group and proposal, then retry only after correcting the input. |
| 10 | `AlreadyPaid` | Treasury operation | AlreadyPaid guard | "This member has already paid for this period." | Review the group and proposal, then retry only after correcting the input. |
| 11 | `AlreadyApproved` | Treasury operation | AlreadyApproved guard | "This signer has already approved the proposal." | Review the group and proposal, then retry only after correcting the input. |
| 12 | `ClosedProposal` | Treasury operation | ClosedProposal guard | "This proposal is already executed or cancelled." | Review the group and proposal, then retry only after correcting the input. |
| 13 | `BelowThreshold` | Treasury operation | BelowThreshold guard | "The proposal needs more signer approvals." | Review the group and proposal, then retry only after correcting the input. |
| 14 | `InsufficientBalance` | Treasury operation | InsufficientBalance guard | "The group does not have enough recorded balance." | Review the group and proposal, then retry only after correcting the input. |
| 15 | `Overflow` | Treasury operation | Overflow guard | "The operation would exceed a numeric limit." | Review the group and proposal, then retry only after correcting the input. |
| 16 | `InvalidRecipient` | Treasury operation | InvalidRecipient guard | "Choose a recipient other than the treasury contract." | Review the group and proposal, then retry only after correcting the input. |
| 17 | `InvalidCancellation` | Treasury operation | InvalidCancellation guard | "Cancellation requires the proposer or a unique threshold of group signers." | Review the group and proposal, then retry only after correcting the input. |
