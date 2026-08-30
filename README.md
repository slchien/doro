# Doro

Doro is an AI agent router for MCP. Give your AI Agent MCP access to tools such as Notion, Slack, Jira, etc. and Doro will intelligently route your agent to them, saving token cost by finding the most efficient route possible.

We aim to reduce a model's context window when searching through tools by upwards of 50%, and all of our code is written in Rust.

## Install

```
bun install -g doro
```

Or try npm, brew, etc.!

## Quickstart

Point Doro at your MCP servers in a config file, then point any model at Doro instead of your tools directly.

```json
{
  "servers": {
    "notion": { "command": "npx", "args": ["-y", "@notionhq/notion-mcp-server"] },
    "slack": { "command": "npx", "args": ["-y", "@modelcontextprotocol/server-slack"] },
    "jira": { "command": "npx", "args": ["-y", "mcp-server-jira"] }
  }
}
```

```
doro serve --config doro.json
```

Your agent now sees one Doro tool instead of the full set from Notion, Slack, and Jira combined.

## Use

Doro acts as an AI agent router -> point any model at it (Anthropic, OpenAI, Kimi) and watch it work. No other work is needed beyond configuring your underlying MCP servers as shown above.

## How it works

The model always sees one single tool, and Doro only has four actions underneath it:

- **search** — the agent describes intent ("find the Q3 roadmap doc"), Doro returns candidate tools ranked by relevance.
- **do** — the agent hands Doro a natural-language task, and Doro resolves intent to tool to validated call in one step when Doro is confident about the match.
- **call** — a direct, validated call to a specific tool once resolved, args checked against the real schema before anything fires.
- **result** — the outcome comes back, big payloads collapsed to a preview plus a handle.

Example: agent asks "who's assigned to the login bug in Jira" -> Doro's `search` matches intent to the Jira `get_issue` tool -> `do` resolves and validates the call -> `call` executes it -> `result` returns the assignee plus a handle to the full issue if the agent wants more.

If Doro isn't confident about the match, it falls back to the original model's tool-selection process, so you get the same, if not better, tool efficiency rather than a bad guess.

## Benefits

**Selection is intelligent.** Doro resolves intent to tool to validated call in one step when confident. Otherwise it falls back to the model's own reasoning over the full tool list.

**The model always sees one single tool.** Add any number of MCP servers and any number of tools — the agent's context surface stays constant.

**It is fast.** Written in Rust. Routing adds under a millisecond; start time is under fifty milliseconds.

**Big results stay out of context.** A forty megabyte query result becomes a small preview plus a handle the agent can page through.

## Compatibility

Tested with Claude Desktop, Claude Code, and Cursor. Works with any MCP-compatible host.

## License

Doro is open source under the MIT license, and allows anyone to access, distribute, and use freely.
