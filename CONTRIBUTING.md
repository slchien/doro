# Contributing to doro

Thanks for your interest. doro is MIT licensed and intends to stay that way.
The router, the CLI, and everything in this repository are and will remain open
source.

## Where the project is right now

doro is in the design phase. There is no code yet. The most valuable
contributions today are design review and real-world requirements. Read
[README.md](README.md) and [docs/DESIGN.md](docs/DESIGN.md), then open an issue
if you see a flaw, a missing use case, or a simpler way to do something. Tell
us how many MCP servers you run, which agents you use, and what breaks for you
today. That shapes the roadmap more than anything else.

## How to contribute

Open an issue first for anything larger than a typo fix. It saves everyone time
if we agree on the direction before you write the change. For small fixes, a
pull request with a clear description is enough.

Keep pull requests focused on one thing. A good description says what changed
and why, in plain language. Write commit messages the same way.

Once code lands, the standard flow will apply: fork the repository, create a
branch, make your change, run the tests, and open a pull request against
`main`. The design doc describes the testing philosophy, including token-budget
tests that fail CI arithmetically if the default context surface grows. A pull
request that inflates the surface needs a very good reason.

## Design principles to keep in mind

Tokens are the scarce resource, and the default surface stays small no matter
what. One writer per piece of state, communicating by messages. Never fake
success on an ambiguous outcome. Log first, acknowledge second. Secrets never
serialize toward an agent. If a change fights one of these, raise it in an
issue rather than working around it.

## Conduct

Be kind and be direct. We follow the spirit of the
[Contributor Covenant](https://www.contributor-covenant.org/). Harassment of
any kind is not tolerated. Disagree with ideas, not with people.

## License

By contributing, you agree that your contributions are licensed under the MIT
license that covers this project.
