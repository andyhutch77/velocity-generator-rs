# Setup guide

The one-off, mostly manual setup behind this repo: accounts, repository settings,
security, deployment and the agent tooling. Everything here was done by hand by a
maintainer, not by an agent. Repository settings aren't code, so they can't go
through a pull request, and changing them is deliberately a human-only job.

Do the steps in order. Each section says **what** was set, and **why**.

## 1. Accounts and plans

| What | Why |
| --- | --- |
| **GitHub Copilot Pro** on the maintainer's account | Agents run on the Copilot engine and spend the plan's monthly AI credits. GitHub Pro is a separate plan and doesn't include Copilot. |
| No extra-usage budget, at first | With no budget set, Copilot stops when the monthly allowance runs out instead of charging more. Top up, or add a budget, under **Settings → Billing** if you need more. |

Check the plan at **github.com/settings/copilot** and under **Settings → Billing**.
`gh aw logs` shows how many AI credits each agent run used.

## 2. Local tools

| Tool | Install | Used for |
| --- | --- | --- |
| GitHub CLI | [cli.github.com](https://cli.github.com), then `gh auth login` | Labels, issues, secrets, PRs |
| gh-aw extension | `gh extension install github/gh-aw` | Compiling the agent workflows |
| Rust | [rustup.rs](https://rustup.rs) | `rust-toolchain.toml` installs the right version and the `wasm32` target automatically |

On Windows, if `gh aw` fails with `No such file or directory`, the install was
incomplete: `gh extension remove gh-aw`, then install it again.

No local Rust? A **Codespace** (**Code → Codespaces → Create codespace on main**) has
the GitHub CLI signed in already. Install Rust there with rustup.

## 3. Create the repository

```
gh repo create velocity-generator-rs --public
```

**Public**, because CodeQL, secret scanning, dependency review, build attestations,
deployment approvals and Actions minutes are all free for public repositories.

The first commit (the README) went straight to `main`, before branch protection
existed. Every change since has gone through a pull request.

## 4. Labels and bootstrap issues

```
./scripts/bootstrap.ps1
```

Creates the six workflow labels (`needs-triage`, `needs-plan`, `plan-review`,
`plan-approved`, `in-progress`, `blocked`) and an issue for each bootstrap step. It's
safe to run again. The agents add `agent`, `rust`, `frontend` and `architecture`
themselves when they first use them.

## 5. Protect `main` with a ruleset

**Settings → Rules → Rulesets → New branch ruleset**, named `protect-main`, status
**Active**, targeting the **default branch**.

| Rule | Setting | Why |
| --- | --- | --- |
| Bypass list | Repository admin | Lets the maintainer merge their own PRs (you can't approve your own) |
| Restrict deletions | On | `main` can't be deleted |
| Block force pushes | On | History on `main` can't be rewritten |
| Require a pull request | 1 approval | Nothing reaches `main` unreviewed |
| Dismiss stale approvals on new commits | On | A `/fix` push after approval needs approving again |
| Require review from Code Owners | On | Changes under `.github/` need the maintainer (see `CODEOWNERS`) |
| Require approval of the most recent push | On | Someone other than the last pusher must approve |
| Require conversation resolution | On | Reviewer and architect comments must be dealt with |
| Allowed merge methods | Squash only | One commit per PR on `main`, so a bad change reverts cleanly |
| Require status checks to pass | `Format and clippy`, `Test`, `Build WebAssembly`, `Cargo deny`, `Dependency review` | CI must be green. Checks can only be picked once they've run at least once |
| Require code scanning results | CodeQL: errors, and security alerts of high or above | New CodeQL findings block merging |

Worth tightening, compared with the current settings:

- **Add `E2E tests`** to the required checks. It runs on every PR but isn't required yet.
- **Bypass mode is "Always".** Setting the admin bypass to **For pull requests only**
  still lets you merge your own PRs, but stops direct pushes to `main`.
- **"Require branches to be up to date before merging"** is off. Turning it on
  re-tests each PR against the latest `main`, so two PRs that each pass can't combine
  into a broken `main`.

`.github/CODEOWNERS`:

```
/.github/ @andyhutch77
```

## 6. Security

**Settings → Advanced Security:**

| Setting | Why |
| --- | --- |
| Dependabot alerts | Flags known-vulnerable dependencies |
| Dependabot version updates | `.github/dependabot.yml`: weekly PRs for Cargo crates and GitHub Actions |
| Secret scanning with push protection | Blocks commits that contain tokens or keys |
| CodeQL code scanning, **Default** setup | Detects the languages itself: Rust, GitHub Actions, and the TypeScript e2e tests |

Dependency rules live in `deny.toml`, checked by `cargo deny` in CI. It allows the
`CC0-1.0` and `BSL-1.0` (Boost) licences, and has one reasoned exception:
`RUSTSEC-2024-0436`, because `paste` is unmaintained (not vulnerable) and comes in
through Leptos.

`Cargo.lock` is committed (`cargo generate-lockfile`), so builds are reproducible and
dependency review can see version changes.

## 7. GitHub Actions

**Settings → Actions → General → Workflow permissions:**

| Setting | Why |
| --- | --- |
| **Read repository contents permission** | Workflows are read-only unless they ask for more |
| **Allow GitHub Actions to create and approve pull requests** | Needed for agents to open PRs. Without it, gh-aw falls back to opening an issue with a link to the branch. No agent has an "approve" output, so approvals stay human |

## 8. GitHub Pages and deploy approval

1. **Settings → Pages:** source **GitHub Actions**.
2. **Settings → Environments → `github-pages`:**
   - **Required reviewers:** the maintainer, so every deploy waits for approval;
   - **Prevent self-review:** off, or you can't approve your own deploys;
   - **Deployment branches and tags:** `main` only.

The site is served from `/velocity-generator-rs/`, so `deploy.yml` builds with
`--public-url "/velocity-generator-rs/"`.

## 9. Copilot token for the agents

Agents in a personal repository authenticate to Copilot with a personal access token.

1. **Settings → Developer settings → Personal access tokens → Fine-grained tokens →
   Generate new token.**
   - Name: `gh-aw-velocity` (just a label)
   - Expiry: up to 90 days, or a fixed date
   - Repository access: public repositories
   - **Account permissions → Copilot Requests**
2. Store it as a repository secret. The secret's **name** must be exactly
   `COPILOT_GITHUB_TOKEN`, because that's what the workflows ask for:

   ```
   gh secret set COPILOT_GITHUB_TOKEN
   ```

   It prompts for the value, so the token never lands in your shell history.

**Renew it before it expires.** An expired token makes every agent fail at once with
`models fetch returned 401` / "model catalog unavailable". Regenerate the token and run
`gh secret set COPILOT_GITHUB_TOKEN` again. Keep only one token, and put its expiry in
your calendar.

## 10. GitHub Agentic Workflows

```
gh aw init --engine copilot
```

Run once, on a branch. It adds the Copilot setup files under `.github/`. Review what it
creates before committing.

After editing any agent's `.md` file:

```
gh aw compile --strict
```

Commit the `.md` and its generated `.lock.yml` together. When an agent uses a new
third-party Action, the compiler asks for a security review: check the Action, add
`--approve`, and note in the PR what you approved and why.

## 11. Optional repo tidying

- **Settings → General → Pull Requests:** the ruleset already enforces squash merges;
  you can also untick merge commits and rebase merging here so the button only offers
  squash.
- **Automatically delete head branches:** currently off. Turning it on removes agent
  branches once their PRs are merged.
