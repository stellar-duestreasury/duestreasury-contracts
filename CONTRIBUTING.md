# Contributing

Thanks for helping with `duestreasury`. **Read [AGENTS.md](AGENTS.md) first** — it is
the rulebook for this repository, for people and for AI agents alike. This page
is a shorter orientation.

## Before you change anything

- **Testnet only.** Never write anything that suggests mainnet use.
- **Nothing is deployed and no pilot has happened.** Do not describe a
  deployment, a contract id, a user, a tester or a result that does not exist.
- **No personal data, ever** — not in examples, not in tests, not in issue
  drafts. Use obvious placeholders.
- **Never commit `.env`**, a secret key or a seed phrase.

## Commits

- One logical change per commit; subject `type: imperative summary`, 72
  characters or fewer.
- Stage by explicit file name and read the staged diff before committing.
- No "Generated with" or co-author trailers of any kind.

## Checks to run before you push

Run every check in [TESTING.md](TESTING.md): formatting, all-target clippy,
Rust tests, Node checker tests, error synchronization and Wasm build. CI
runs the same checks. Keep the deployment block and second human custody
review requirement intact; local tests do not authorize funded testing.
