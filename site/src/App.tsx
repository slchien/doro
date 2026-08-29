import { useState } from "react"
import { Input } from "@base-ui/react/input"

const REPO = "slchien/doro"
const SEALED_TOKEN =
  "B73HknJQXFlWJQCccS-s-hMNEjyOM0H7bWKupaytnG4kzkgVtaKWpBb2MWPh_Kki5iH-UhPIpWN_L1tLyojFm7Ub8l5sBZ2WvkKoVzBw3wjKbvjYRMIjaThrR-d1amJfRBcTUSasII8JAfRK15EAutq-prjujrdyJznwgLqqbFNvRJTXw5U6m1OdZipU"
const CHART_URL = `https://api.star-history.com/chart?repos=${REPO}&type=date&theme=dark&legend=top-left&sealed_token=${SEALED_TOKEN}`

// Formspree endpoint for waitlist storage. Empty until the form is created.
const FORM_ENDPOINT = ""

const TITLE = `
██████╗  ██████╗ ██████╗  ██████╗
██╔══██╗██╔═══██╗██╔══██╗██╔═══██╗
██║  ██║██║   ██║██████╔╝██║   ██║
██║  ██║██║   ██║██╔══██╗██║   ██║
██████╔╝╚██████╔╝██║  ██║╚██████╔╝
╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝
`.slice(1)

const features = [
  { title: "One tool", text: "The model sees a single tool, no matter how many you connect." },
  { title: "One place", text: "Rules and keys for every agent, out of the context window." },
  { title: "Fast", text: "One Rust binary. Under a millisecond per call." },
]

function GitHubIcon() {
  return (
    <a
      href={`https://github.com/${REPO}`}
      aria-label="doro on GitHub"
      className="absolute right-6 top-8"
    >
      <svg viewBox="0 0 16 16" className="size-8 fill-zinc-50 transition-colors hover:fill-zinc-400">
        <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z" />
      </svg>
    </a>
  )
}

function Waitlist() {
  const [email, setEmail] = useState("")
  const [status, setStatus] = useState<"idle" | "sending" | "done" | "error">("idle")

  async function submit(e: React.FormEvent) {
    e.preventDefault()
    if (!FORM_ENDPOINT) {
      setStatus("error")
      return
    }
    setStatus("sending")
    try {
      const res = await fetch(FORM_ENDPOINT, {
        method: "POST",
        headers: { Accept: "application/json", "Content-Type": "application/json" },
        body: JSON.stringify({ email }),
      })
      setStatus(res.ok ? "done" : "error")
    } catch {
      setStatus("error")
    }
  }

  if (status === "done") {
    return <p className="py-2 font-mono text-zinc-50">You are on the list.</p>
  }

  return (
    <form onSubmit={submit} className="flex w-full max-w-md flex-col items-center gap-2">
      <div className="flex w-full gap-2">
        <Input
          type="email"
          required
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          placeholder="you@example.com"
          className="h-11 w-full rounded-lg border border-zinc-800 bg-zinc-900 px-4 font-mono text-base text-zinc-50 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none"
        />
        <button
          type="submit"
          disabled={status === "sending"}
          className="h-11 shrink-0 rounded-lg bg-zinc-50 px-5 font-medium text-zinc-950 transition-colors hover:bg-zinc-300 disabled:opacity-60"
        >
          {status === "sending" ? "Joining..." : "Join waitlist"}
        </button>
      </div>
      {status === "error" && (
        <p className="text-sm text-zinc-50">The waitlist is not open yet. Check back soon.</p>
      )}
    </form>
  )
}

export default function App() {
  return (
    <main className="relative mx-auto flex min-h-svh max-w-3xl flex-col justify-center gap-7 px-6 py-8">
      <GitHubIcon />

      <section className="flex flex-col items-center gap-4 text-center">
        <h1 className="sr-only">doro</h1>
        <pre aria-hidden className="font-mono text-[11px] leading-[1.1] text-zinc-50 sm:text-lg">
          {TITLE}
        </pre>
        <p className="max-w-xl text-balance text-lg text-zinc-50">
          An MCP router that saves hundreds of thousands of tokens by
          compressing tool use.
        </p>
        <Waitlist />
      </section>

      <section>
        <img
          alt="Star History Chart"
          src={CHART_URL}
          className="mx-auto max-h-[36svh] rounded-xl border border-zinc-800 bg-zinc-900/50"
        />
      </section>

      <section className="flex justify-center gap-4 overflow-x-auto">
        {features.map((f) => (
          <pre key={f.title} className="font-mono text-xs leading-snug text-zinc-50">
            {`┌─${"─".repeat(26)}─┐\n` +
              [f.title, "", ...wrap(f.text, 26)].map((r) => `│ ${r.padEnd(26)} │`).join("\n") +
              `\n└─${"─".repeat(26)}─┘`}
          </pre>
        ))}
      </section>
    </main>
  )
}

function wrap(text: string, width: number): string[] {
  const lines: string[] = []
  let line = ""
  for (const word of text.split(" ")) {
    if (line && line.length + word.length + 1 > width) {
      lines.push(line)
      line = word
    } else {
      line = line ? `${line} ${word}` : word
    }
  }
  if (line) lines.push(line)
  return lines
}
