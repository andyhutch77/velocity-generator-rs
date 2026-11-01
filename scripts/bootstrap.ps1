<#
.SYNOPSIS
    Creates the agentic workflow labels and the bootstrap tracking issues.

.DESCRIPTION
    Safe to run more than once: labels are created or updated with --force,
    and an issue is only created if an open or closed issue with the same
    title doesn't already exist.

    Requires the GitHub CLI (gh), logged in with access to this repo.
    Run from anywhere inside the repo clone, or pass -Repo owner/name.

.EXAMPLE
    ./scripts/bootstrap.ps1
    ./scripts/bootstrap.ps1 -Repo andyhutch77/velocity-calculator-rs
#>
[CmdletBinding()]
param(
    [string]$Repo
)

$ErrorActionPreference = 'Stop'

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    throw "GitHub CLI (gh) not found. Install it from https://cli.github.com and run 'gh auth login'."
}

$repoArgs = @()
if ($Repo) { $repoArgs = @('--repo', $Repo) }

# --- Labels: issue state for the agentic workflow -------------------------

$labels = @(
    @{ Name = 'needs-triage';  Color = 'fbca04'; Description = 'New issue, not yet looked at' }
    @{ Name = 'needs-plan';    Color = '1d76db'; Description = 'Ready for an agent plan (human-set)' }
    @{ Name = 'plan-review';   Color = '5319e7'; Description = 'Plan posted, waiting on human review' }
    @{ Name = 'plan-approved'; Color = '0e8a16'; Description = 'Approved to build (human-set)' }
    @{ Name = 'in-progress';   Color = 'c5def5'; Description = 'Agent PR open' }
    @{ Name = 'blocked';       Color = 'd93f0b'; Description = 'Agent needs an answer' }
)

Write-Host "Creating labels..." -ForegroundColor Cyan
foreach ($label in $labels) {
    gh label create $label.Name --color $label.Color --description $label.Description --force @repoArgs | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "Failed to create label '$($label.Name)'." }
    Write-Host "  $($label.Name)"
}

# --- Bootstrap issues: the hand-built steps before agents take over ------

$issues = @(
    @{
        Title = 'Protect main with a ruleset and CODEOWNERS'
        Body  = @"
Bootstrap step 1.

- [ ] Branch ruleset on ``main``: require a PR with 1 approval, require status checks, block force pushes
- [ ] Add ``.github/CODEOWNERS`` so changes under ``.github/`` need human review
- [ ] Dependabot alerts on; secret scanning with push protection on
- [ ] Pages source set to GitHub Actions
- [ ] Actions default workflow permissions set to read-only
"@
    }
    @{
        Title = 'Add CI: build, test, CodeQL, dependency review'
        Body  = @"
Bootstrap step 2.

- [ ] ``cargo build`` and ``cargo test``
- [ ] clippy and rustfmt
- [ ] CodeQL code scanning for Rust
- [ ] Dependency review and ``cargo deny``
- [ ] Add the checks as required in the ``main`` ruleset
"@
    }
    @{
        Title = 'Install gh-aw and add the Copilot token secret'
        Body  = @"
Bootstrap step 3.

- [ ] ``gh extension install github/gh-aw``
- [ ] ``gh aw init``
- [ ] Add the ``COPILOT_GITHUB_TOKEN`` secret (fine-grained PAT)
"@
    }
)

Write-Host "Creating bootstrap issues..." -ForegroundColor Cyan
$existing = gh issue list --state all --limit 200 --json title @repoArgs | ConvertFrom-Json
$existingTitles = @($existing | ForEach-Object { $_.title })

foreach ($issue in $issues) {
    if ($existingTitles -contains $issue.Title) {
        Write-Host "  Skipped (already exists): $($issue.Title)" -ForegroundColor DarkGray
        continue
    }
    $url = gh issue create --title $issue.Title --body $issue.Body @repoArgs
    if ($LASTEXITCODE -ne 0) { throw "Failed to create issue '$($issue.Title)'." }
    Write-Host "  $url"
}

Write-Host "Done." -ForegroundColor Green
