# Treasury event layout

`TreasuryEvent` in `src/types.rs` has topics `treasury_event` (the event name), `group_id: u64`, `action: Symbol`, and data `record_id: u64`, `actor: Address`, `amount: i128`. Actions are `created`, `dues`, `proposed`, `approved`, `executed`, `cancelled`. Dues records use the period as record_id; proposal actions use proposal id. No free text or personal data is emitted. Event publication is tested immediately after each invocation in the lifecycle suite, because the SDK test event API reports the last invocation.
