"""Add numbered highlight boxes and a key to walkthrough screenshots.

Usage: python3 scripts/annotate-walkthrough.py <raw_screenshots_dir> docs/walkthrough
Requires Pillow (pip install pillow).
Boxes are in the screenshot's own pixel coordinates: (x0, y0, x1, y1).
"""
import sys, textwrap
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ACCENT = (232, 89, 12)        # orange: stands apart from GitHub's blues, greens and purples
INK = (31, 35, 40)
FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
BOLD = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"
SCALE = 2                     # draw at 2x, then downsample, for smooth edges

ANNOTATIONS = {
    "01-issue.png": [
        ((246, 76, 730, 124), "A new issue. The title is all it says."),
        ((252, 232, 470, 270), "The description: too vague to build from yet."),
        ((248, 128, 414, 164), "Later closed automatically by the front-end PR, #71."),
    ],
    "02-triage-blocked.png": [
        ((70, 5, 348, 33), "The triage agent adds the blocked label."),
        ((40, 134, 842, 226), "Its questions about what's missing."),
        ((44, 270, 608, 302), "Every agent comment says which workflow wrote it, the model, and the AI credits used."),
    ],
    "03-reply-retriage.png": [
        ((34, 92, 802, 176), "You answer the questions on the issue."),
        ((32, 248, 362, 394), "Back then, closing and reopening re-ran triage. Now a reply to a blocked issue re-runs it automatically."),
        ((70, 426, 376, 456), "Triage marks it ready to plan."),
        ((42, 590, 832, 642), "Its suggested type and the parts of the code involved."),
        ((42, 650, 374, 678), "Next step is yours: add needs-plan."),
    ],
    "04-plan.png": [
        ((64, 20, 480, 48), "You add needs-plan, which starts the planner."),
        ((64, 62, 486, 92), "The planner swaps it for plan-review once the plan is up."),
        ((40, 258, 842, 470), "Success criteria that can be checked, ticked off later in the PRs."),
        ((50, 583, 842, 736), "Tasks tagged [rust] or [frontend]: this is how the orchestrator knows which agent does what."),
    ],
    "05-architect.png": [
        ((54, 158, 242, 188), "The architect's verdict on the plan."),
        ((54, 194, 832, 238), "Why, judged against ARCHITECTURE.md and the decision records."),
        ((50, 18, 507, 48), "Then you approve: adding plan-approved is the first human gate."),
    ],
    "06-progress-stage1.png": [
        ((36, 114, 406, 216), "Stage table: the Rust agent is started first, the front end waits."),
        ((34, 222, 839, 265), "What the orchestrator just did, and why."),
        ((34, 275, 839, 319), "What happens next, and whether it's waiting on you."),
        ((350, 32, 472, 54), "Each new progress update hides the previous one."),
    ],
    "07-agent-pr.png": [
        ((224, 68, 782, 114), "Agent PRs are titled [agent]. Only these get the reviewer and /fix."),
        ((510, 163, 602, 194), "CI ran: 28 checks."),
        ((282, 256, 364, 284), "Part of #68, not Closes: merging this won't close the issue while front-end work remains."),
        ((282, 338, 992, 392), "The plan's [rust] tasks, ticked off."),
        ((282, 753, 782, 856), "Front-end tasks left for the next agent."),
    ],
    "08-checks-review.png": [
        ((154, 74, 280, 94), "The reviewer can only comment. It can never approve or block."),
        ((68, 114, 792, 250), "A summary against the plan, the architecture, the tests and accessibility."),
        ((68, 256, 792, 302), "The one thing it thinks you should check before approving."),
    ],
    "09-line-comment.png": [
        ((96, 120, 412, 142), "The line it's commenting on."),
        ((114, 192, 772, 234), "What's wrong, the rule it breaks, and how to fix it, worded to work with /fix."),
    ],
    "10-fix.png": [
        ((58, 50, 182, 82), "You ask for changes with /fix."),
        ((72, 202, 402, 264), "The fix agent runs and pushes a commit."),
        ((108, 376, 797, 402), "The new commit lands on the same PR."),
        ((72, 514, 792, 670), "What it changed for each piece of feedback."),
    ],
    "11-final-progress.png": [
        ((93, 4, 357, 28), "The front-end PR #71 is merged..."),
        ((58, 40, 412, 68), "...which closes the issue."),
        ((68, 184, 368, 280), "The orchestrator's final update: both stages merged."),
        ((68, 288, 847, 334), "Nothing left to start. (It now also clears the issue's workflow labels here.)"),
    ],
    "12-deploy-approval.png": [
        ((318, 373, 558, 417), "The app is built and its build attestation signed."),
        ((300, 190, 920, 240), "Then the deploy waits for you: the third human gate."),
        ((602, 373, 840, 440), "The github-pages environment, waiting for review."),
    ],
    "13-live-app.png": [
        ((64, 470, 392, 536), "The Number of days off input from #68, live on GitHub Pages.", False),
    ],
}


def font(path, size):
    return ImageFont.truetype(path, size * SCALE)


def annotate(src: Path, dst: Path, items):
    base = Image.open(src).convert("RGB")
    w, h = base.size
    S = SCALE
    big = base.resize((w * S, h * S), Image.LANCZOS)

    # Key below the screenshot.
    key_font, num_font = font(FONT, 14), font(BOLD, 13)
    line_h, pad, badge_r = 21, 14, 11
    wrap_chars = max(40, int((w - 2 * pad - 34) / 7.6))
    key_lines = []
    for i, (_, label, *_) in enumerate(items, 1):
        wrapped = textwrap.wrap(label, wrap_chars)
        key_lines.append((i, wrapped))
    key_h = pad * 2 + sum(max(1, len(l)) * line_h + 6 for _, l in key_lines)

    out = Image.new("RGB", (w * S, (h + key_h) * S), "white")
    out.paste(big, (0, 0))
    d = ImageDraw.Draw(out)

    def badge(cx, cy, n):
        r = badge_r * S
        d.ellipse((cx - r, cy - r, cx + r, cy + r), fill=ACCENT, outline="white", width=2 * S)
        d.text((cx, cy), str(n), font=num_font, fill="white", anchor="mm")

    # Highlight boxes with a badge on the top-left corner.
    for i, ((x0, y0, x1, y1), _, *opt) in enumerate(items, 1):
        if not opt or opt[0]:   # False = badge only (the screenshot is already marked)
            d.rounded_rectangle((x0 * S, y0 * S, x1 * S, y1 * S), radius=6 * S, outline=ACCENT, width=3 * S)
        cx = max(badge_r + 2, x0) * S
        cy = max(badge_r + 2, y0) * S
        badge(cx, cy, i)

    # Separator and key.
    d.line((0, h * S, w * S, h * S), fill=(208, 215, 222), width=1 * S)
    y = h + pad
    for i, wrapped in key_lines:
        badge((pad + badge_r) * S, (y + line_h // 2) * S, i)
        for j, text in enumerate(wrapped):
            d.text(((pad + 2 * badge_r + 10) * S, (y + j * line_h + 2) * S), text, font=key_font, fill=INK)
        y += max(1, len(wrapped)) * line_h + 6

    out = out.resize((w, h + key_h), Image.LANCZOS)
    out.save(dst, optimize=True)


if __name__ == "__main__":
    src_dir, dst_dir = Path(sys.argv[1]), Path(sys.argv[2])
    dst_dir.mkdir(parents=True, exist_ok=True)
    for name, items in ANNOTATIONS.items():
        annotate(src_dir / name, dst_dir / name, items)
        print("annotated", name)
