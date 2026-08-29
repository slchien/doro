# doro design

Everything about how doro works, on one page.

## The idea

Agents waste context on tool schemas they never use. doro turns the tool
catalog into something the agent queries instead of something it reads.

The tools you use constantly can be pinned. Pinned tools are exposed as normal
MCP tools with no indirection and no overhead. Everything else sits behind one
tool named `doro` with four actions. The `do` action takes plain intent, such
as "file a github issue" plus arguments, picks the right tool, and runs it. If
the match is ambiguous it asks the agent to choose. The `search` action returns
five ranked one-line matches. The `call` action runs a specific tool by
qualified name, such as `sentry.list_issues`. The `result` action pages through
a big stored result without reloading it.

One tool means the context surface is constant. Connect a thousand tools and
the model still reads one schema.

The agent never needs a tool's full schema up front. If it calls with wrong
arguments, doro validates locally and returns just the broken fragment of the
schema, a few tokens, so the agent fixes it in one turn.

## Picking the right tool

Selection is a ladder, and doro climbs as high as confidence allows.

The floor is a ranked menu. Search returns the top five candidates and the
agent chooses. Worst case is still five options instead of a thousand.

When the top match dominates the runner-up by a clear score margin, doro
returns exactly one tool, or through the `do` action simply runs it. Below the
threshold it falls back to the menu and says why it is unsure.

Every outcome feeds back into ranking, per profile. A tool that gets chosen and
succeeds gains weight. One that errors or gets skipped loses it. Your daily
patterns become near-instant top picks. Frequent intents can also be aliased
explicitly, for example `doro alias "deploy docs" gh-pages.deploy`.

For large catalogs there is an optional sharper ranker: a small local
cross-encoder model that re-ranks the candidates in about ten milliseconds with
no API calls.

The whole ladder is deterministic. Scoring, thresholds, and learned weights,
with no language model inside the router. Same input, same pick, and
`doro why <query>` shows exactly how a pick was scored.

There is a line doro will not cross. It picks which tool, but never invents
what to send. Arguments come from the agent, which has the conversation
context. And it executes one call per request, never chaining its own
multi-step plans. That keeps doro fast, debuggable, and policy-checkable.
Planning stays in the agent, where the intelligence already is.

## Architecture

```
  agents (Claude Code, Codex, ...)
        │  one MCP endpoint, stdio or HTTP
        ▼
┌───────────────────────── doro ─────────────────────────┐
│  sessions ──▶ router ──▶ policy check ──▶ dispatch      │
│                 │                                       │
│    ┌────────────┼──────────────┬─────────────┐          │
│    ▼            ▼              ▼             ▼          │
│  search      result store   secrets      audit log      │
│  index       big outputs    vault        every call,    │
│  BM25        → handles      keychain     token stats    │
└───────┬───────────┬───────────┬────────────────────────┘
        ▼           ▼           ▼
     github       slack      postgres   ...  your MCP servers
```

One Rust process, one binary. State lives in local files under
`~/.local/share/doro/`. No database, no daemon mesh, no cloud.

### The pieces

The router resolves names like `github.create_issue`, validates arguments, and
dispatches. Each upstream server gets its own supervised connection with a
timeout, a concurrency cap, and a circuit breaker, so one wedged server cannot
jam the rest. Servers you have not pinned are not even started until first use.

The search index is full-text BM25 over compressed one-line tool cards. It
re-indexes automatically when a server's tools change. Local embeddings may be
added later for fuzzier matching, but BM25 works with zero machine learning.

Policy is a per-tool pattern set to allow, ask, or deny. An ask parks the call
until you approve it with `doro approvals`. Denied tools are invisible to
search, so an agent cannot discover what it cannot use. Profiles let CI run
default-deny while your laptop runs default-allow.

The vault keeps credentials in the OS keychain and injects them into servers.
They are wrapped in a type that redacts on serialization, so they are
structurally unable to leak toward an agent.

The result store writes outputs over about 2 KB to disk. The agent gets a
preview plus a handle. Big intermediates never enter model context.

The audit log is an append-only file of every call, every decision, and its
token cost. `doro tokens` reads it. If doro crashes, it rebuilds all other
state from the log on restart.

### Rules the code follows

Tokens are the scarce resource. The default surface must stay under about 800
tokens no matter how many servers you add. This is a CI test, not a guideline.

One writer per piece of state. Each session and each upstream is a single async
task that owns its state, and everything else sends it messages. No shared
locks, no races.

Never fake success. If a write times out mid-flight, doro reports an unknown
outcome. It retries automatically only for tools marked read-only or
idempotent.

Log first, ack second. Nothing is confirmed to an agent before it is in the
audit log.

These are the boring, proven patterns from *Designing Data-Intensive
Applications* and *Release It!*: single-writer state, write-ahead logging,
bulkheads, and circuit breakers.

## Why Rust and why bun install

doro sits inside every tool call, and agents spawn it fresh per session. So the
two numbers that matter are added latency, under a millisecond, and cold start,
under fifty milliseconds. Rust gives both: no garbage collector, no runtime,
one static binary.

Distribution is npm-style. `bun install -g doro` downloads a prebuilt binary
for your platform, the same trick esbuild and biome use. It will also be on
cargo and Homebrew. The language of the core is invisible to users.

## Speed and size targets

| | target |
|---|---|
| routing overhead, p99 | under 1 ms |
| search over 10k tools | under 30 ms |
| cold start | under 50 ms |
| memory with 40 servers | under 150 MB |
| default context surface | 800 tokens or less |

## Plan

M0, about two weeks, is pass-through. Speak MCP in both directions and proxy
one server transparently. Claude Code works through it with zero difference.

M1, about four weeks, is the router. Many servers, pinning, profiles, allow and
deny policy, the vault, and the audit log. It must survive killing any server
mid-call.

M2, about four weeks, is disclosure and selection. The single `doro` tool with
its four actions, confident-pick thresholds, outcome-weighted ranking, the
approval queue, and the result store. This is the release that matters. The
demo is a real session with top picks on common intents and at least 90 percent
fewer context tokens.

M3, about three weeks, is hardening. A long-lived HTTP mode so multiple agents
share one instance, fault-injection tests, and a performance pass.

M4, about two weeks, is v0.1. `bun install -g doro` works on a clean machine in
under ten minutes. MIT licensed, with public CI running the token-budget and
recall benchmarks so the claims are reproducible by anyone.

Some things are deliberately not being built. doro routes but does not host or
run agents. It requires no custom SDK, just plain MCP. It routes tools, not
model inference. Later, maybe, a code mode where the agent writes short scripts
against generated typed stubs so multi-step pipelines keep intermediates
entirely out of context.

## Reading list

[RAG-MCP](https://arxiv.org/abs/2505.03275) shows tool search beats tool
loading, with more than 50 percent fewer tokens and three times the selection
accuracy. [MCP-Zero](https://arxiv.org/abs/2506.01056) shows on-demand
discovery with about 98 percent token reduction, and provides the benchmark
dataset we will use for search quality.
[Toolshed](https://arxiv.org/abs/2410.14594) and
[ScaleMCP](https://arxiv.org/abs/2505.06416) cover making tool retrieval
reliable at scale and keeping the index in sync.
[Anthropic's advanced tool use report](https://www.anthropic.com/engineering/advanced-tool-use)
gives production numbers for tool search, around 85 percent, and for keeping
results out of context, one workload going from 150k tokens to 2k. The books
behind the runtime are Kleppmann's *Designing Data-Intensive Applications* and
Nygard's *Release It!*.
