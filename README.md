[![CI](https://github.com/andyhutch77/velocity-generator-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/andyhutch77/velocity-generator-rs/actions/workflows/ci.yml)
[![Deploy](https://github.com/andyhutch77/velocity-generator-rs/actions/workflows/deploy.yml/badge.svg?branch=main)](https://github.com/andyhutch77/velocity-generator-rs/actions/workflows/deploy.yml)
[![CodeQL](https://github.com/andyhutch77/velocity-generator-rs/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/andyhutch77/velocity-generator-rs/security/code-scanning)
[![Live app](https://img.shields.io/badge/live-GitHub%20Pages-2ea44f?logo=github)](https://andyhutch77.github.io/velocity-generator-rs/)

[![Rust](https://img.shields.io/badge/Rust-WebAssembly-dea584?logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/UI-Leptos-ef3939)](https://leptos.dev/)
[![Built with GitHub Agentic Workflows](https://img.shields.io/badge/built%20with-GitHub%20Agentic%20Workflows-6f42c1?logo=githubactions&logoColor=white)](https://github.github.com/gh-aw/)

# Velocity Calculator (Rust)

A Rust rebuild of [Velocity Calculator](https://github.com/andyhutch77/velocity-calculator), compiled to WebAssembly and running entirely in the browser, hosted on GitHub Pages.

Velocity Calculator works out a team's velocity for an upcoming sprint, so it can plan how much work to commit to. The [original version](https://andyhutch77.github.io/velocity-calculator/) is a React app with a retro Windows 98 look. This rebuild matches its behaviour and look, but **no code is carried over**. The UI is written from scratch with a modern Rust front-end framework, and the Windows 98 styling comes from the project's own reusable component library and design tokens.

## Project goal

This repo has two goals:

1. **Rebuild Velocity Calculator in Rust**, as a browser-only WebAssembly app with feature parity with the original, written from scratch.
2. **Build it entirely through GitHub Agentic Workflows.** Every change starts as an issue, an agent plans it, a human approves the plan, agents implement it, and nothing ships without passing CI, security gates and human review.

It's also hands-on practice for the **GitHub GH-600: Agentic AI Developer** exam, so the setup deliberately covers each exam domain: planning before execution, tool and MCP scoping, memory and state, evaluation, multi-agent orchestration, and guardrails.

## Tech stack

| Area | Choice |
| --- | --- |
| Language | Rust, compiled to `wasm32-unknown-unknown` |
| UI framework | Leptos (client-side rendering, fine-grained reactivity) |
| Build | Trunk |
| Hosting | GitHub Pages |
| Styling | Own Windows 98 component library (window, title bar, buttons, inputs) built on CSS custom properties as design tokens; no third-party theme CSS |
| Accessibility | Semantic HTML, keyboard navigation and WCAG AA contrast, despite the retro look |
| Tests | `cargo test`, Playwright end-to-end |
| Agents | GitHub Agentic Workflows (gh-aw) with the GitHub Copilot engine |
| Security | CodeQL, secret scanning with push protection, dependency review, `cargo deny`, artifact attestations |

---

## How the agentic workflow works

### The big picture

There are three human gates: approving the plan, approving the PR, and approving the production deploy. Agents never write to the repo directly. They request actions through **safe outputs**, which run as separate, tightly scoped jobs.

```mermaid
flowchart LR
    A[Issue opened] --> B[Triage agent]
    B --> C[Planner agent]
    C --> D{You approve plan?}
    D -- "no: comment, then /plan" --> C
    D -- "yes: plan-approved" --> E[Orchestrator]
    E --> F[Specialist agents open draft PRs]
    F --> G[Required checks]
    G -- "fail: /fix" --> F
    G -- pass --> H{You approve PR?}
    H -- yes --> I[Merge to main]
    I --> J[Build, attest, smoke test]
    J --> K{You approve deploy?}
    K -- yes --> L[GitHub Pages]
```

### Issue lifecycle (labels)

Labels hold each issue's state. Only a human can apply `needs-plan` and `plan-approved`, the two labels that move work forward.

```mermaid
stateDiagram-v2
    [*] --> needs_triage: issue opened
    needs_triage --> blocked: agent needs an answer
    blocked --> needs_triage: question answered
    needs_triage --> needs_plan: you add needs-plan
    needs_plan --> plan_review: planner posts plan
    plan_review --> plan_review: feedback + /plan
    plan_review --> plan_approved: you add plan-approved
    plan_approved --> in_progress: agent PR opened
    in_progress --> [*]: PR merged
```

| Label | Meaning | Set by |
| --- | --- | --- |
| `needs-triage` | New issue, not yet looked at | Triage agent |
| `needs-plan` | Ready for an agent plan | You |
| `plan-review` | Plan posted, waiting on you | Planner agent |
| `plan-approved` | Go ahead and build it | You |
| `in-progress` | Agent PR open | Implementer agent |
| `blocked` | Agent needs an answer | Any agent |

The planner runs only when a maintainer adds the `needs-plan` label. To request a
revised plan, leave feedback in a comment, then remove and re-add `needs-plan`;
commenting by itself does not trigger the planner. The planner removes
`needs-plan` after posting a plan, so each run requires adding it again.

### Agents and orchestration

When a plan is approved, the orchestrator splits it into tasks and hands each one to a specialist agent on its own branch, so their changes don't collide. The architect agent reviews plans and PRs against `ARCHITECTURE.md` but never writes code.

```mermaid
flowchart TB
    P[Approved plan] --> O[Orchestrator]
    O -- dispatch --> R[Rust agent]
    O -- dispatch --> FE[Front-end design agent]
    R --> PR1[Draft PR: core logic]
    FE --> PR2[Draft PR: UI]
    PR1 --> REV[Reviewer agent]
    PR2 --> REV
    ARCH[Architect agent] -. reviews .-> P
    ARCH -. reviews .-> PR1
    ARCH -. reviews .-> PR2

    subgraph MCP[MCP servers]
        GH[GitHub MCP]
        PW[Playwright MCP]
        VM[velocity-mcp]
    end

    R -.-> GH
    R -.-> VM
    FE -.-> PW
    REV -.-> GH
```

| Agent | Files | Job |
| --- | --- | --- |
| Triage | `.github/workflows/triage.md` | Checks new issues are clear, asks questions |
| Planner | `.github/workflows/plan.md` | Writes a plan with approach, files, tests, risks and success criteria |
| Orchestrator | `.github/workflows/orchestrate.md` | Splits the approved plan and dispatches specialists |
| Rust agent | `.github/agents/rust.agent.md` + `impl-rust.md` | Core logic: idiomatic Rust, clippy clean, tested |
| Front-end design agent | `.github/agents/frontend.agent.md` + `impl-frontend.md` | UI, accessibility, responsive layout |
| Reviewer | `.github/workflows/pr-review.md` | Reviews each PR against its plan |
| Fixer | `.github/workflows/pr-fix.md` | Responds to `/fix` on a PR |
| Architect | `.github/agents/architect.agent.md` + `arch-review.md` | Guards structure and patterns; weekly drift report |

### CI/CD and security gates

Agent PRs face exactly the same required checks as human ones.

```mermaid
flowchart LR
    PR[Pull request] --> B[cargo build + test]
    PR --> L[clippy + rustfmt]
    PR --> E2E[Playwright e2e]
    PR --> CQ[CodeQL]
    PR --> DR[Dependency review + cargo deny]
    PR --> WL[Workflow lint]
    B & L & E2E & CQ & DR & WL --> OK{All green?}
    OK -- yes --> APPROVE[Your approval]
    APPROVE --> MERGE[Merge to main]
    MERGE --> BUILD[trunk build --release]
    BUILD --> SBOM[SBOM + provenance attestation]
    SBOM --> SMOKE[Smoke tests]
    SMOKE --> VERIFY[gh attestation verify]
    VERIFY --> PAGES[Deploy to GitHub Pages]
```

## Guardrails

| Agent action | Risk | Autonomy |
| --- | --- | --- |
| Label or comment on an issue | Low | Acts alone |
| Write or rewrite a plan | Low | Acts alone; you approve before any code |
| Open a draft PR | Medium | Acts alone; merge needs you |
| Merge to `main` | High | Humans only |
| Deploy to Pages | High | Human approves the environment |
| Edit anything under `.github/` | High | Blocked for agents |

- Agents run with **read-only** permissions; all writes go through capped, allow-listed safe outputs.
- `main` is protected by a ruleset: PR required, checks required, one approval, no force pushes.
- `CODEOWNERS` requires human review for anything under `.github/`.
- Safe outputs pass through **agentic threat detection** before they're applied.

## Contributing

Every change starts as an issue. Open one describing what you want, and the triage agent will pick it up. Changes to workflows, agents and repo settings are made by humans only.
