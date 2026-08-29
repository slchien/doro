# doro

**Thousands of tools, no bloat.**

Connect everything you use and doro still shows the model a single tool. It
searches your catalog and loads a tool's schema only when the agent actually
calls it, so the prompt never balloons.

Behind that one tool sits everything you would otherwise wire up per agent:
every MCP server you have, one vault for credentials, one policy layer, one
audit log. Every agent you run shares it, whether that is Claude Code, Codex,
Cursor, or a custom bot. Selection is smart, too. The agent says what it needs,
doro picks the right tool when the match is clear, offers the top five when it
is not, and learns from what succeeds.

## Install

```bash
bun install -g doro
```

npm works as well. One command, one binary. The core is written in Rust for
speed, and the package ships a prebuilt binary for your platform, the same way
esbuild does.

## Use

```bash
doro import ~/.claude/mcp.json
doro pin 'github.*'
doro serve
```

The first command pulls in the servers you already have. The second pins your
hot set, the tools you use all day, so they stay available with zero overhead.
The third starts routing.

Then point any MCP agent at it:

```json
{ "mcpServers": { "doro": { "command": "doro", "args": ["serve"] } } }
```

That is it. No agent changes, no SDK.

## What you get

**Rules live in one place.** Run `doro policy set 'slack.*' ask` and anything
outward facing needs your approval, for every agent at once. A denied tool is
blocked everywhere and does not even show up in search. CI can run a
default-deny profile while your laptop runs default-allow.

**Secrets live in one place.** API keys and OAuth tokens are stored in your OS
keychain, injected into the servers that need them, and never appear in any
agent's context or config file.

**Every call is logged.** What was touched, when, which rule allowed it, and
what it cost in tokens. Run `doro tokens` to read it.

**Selection is smart.** The `do` action resolves intent to tool to validated
call in one step when doro is confident about the match. When it is not, it
returns the best candidates and says what is missing. Selection sharpens the
more you use it.

**The model always sees a single tool.** It is named `doro` and has four
actions: do, search, call, and result. Add ten servers or a hundred and the
context surface stays constant. Pin favorites if you want them exposed as
first-class tools with no indirection.

**It is fast.** Rust, one static binary, no runtime. Routing adds under a
millisecond and cold start is under fifty milliseconds.

**Big results stay out of context.** A forty megabyte query result becomes a
small preview plus a handle the agent can page through.

## Why this works

Deferred tool loading is well studied. [RAG-MCP](https://arxiv.org/abs/2505.03275)
showed that searching tools instead of loading them cuts prompt tokens by more
than half and triples tool selection accuracy.
[MCP-Zero](https://arxiv.org/abs/2506.01056) showed on-demand discovery cutting
token use by about 98 percent across roughly three thousand candidate tools.
[Anthropic reported](https://www.anthropic.com/engineering/advanced-tool-use)
around an 85 percent reduction from tool search in production, and one workload
dropped from 150k tokens to 2k by keeping intermediate results out of context.

Design details, the architecture, and the roadmap live in
[docs/DESIGN.md](docs/DESIGN.md).

## Status

Design phase. No code yet. The design doc is the current deliverable, and
design feedback is the most useful contribution right now. Open an issue if
you see a flaw or a missing use case.

## License

MIT.
