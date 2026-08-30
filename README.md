# Doro

Doro is an AI agent router for MCP. Give your AI Agent MCP access to tools such as Notion, Slack, Jira, etc. 
and Doro will intelligently route your agent to them, saving token cost by finding the most efficient route possible.

We aim to reduce a model's context window when searching through tools by upwards of 50%, and all of our code is written in Rust.

## Install

```bash
bun install -g doro
```
Or try npm, brew, etc.!

## Use

Doro acts as an AI agent harness -> point any model at it (Anthropic, OpenAI, Kimi)
and watch it work. No other work is needed. 

## Benefits

**Selection is intelligent.** 
Doro's action resolves intent to tool to validated
call in one step when doro is confident about the match. If not, it falls back on the 
original model's thought process, finding the same, if not better, tool efficiency. 

**The model always sees one single tool.** 
Doro only has four actions: do, search, call, and result. Add any amount of AI agents
context surface stays constant. 

**It is fast.** Because doro is built in rust, it is the fastest router on the planet.  
Routing adds under a millisecond and start time is under fifty milliseconds.

**Big results stay out of context.** A forty megabyte query result becomes a
small preview plus a handle the agent can page through.


## License

Doro is open source and allows anyone to access, distribute, and use freely. 
