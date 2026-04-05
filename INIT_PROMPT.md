# INIT_PROMPT.md — Claude Code Repository Initialisation Prompt

Paste the content between the cut lines as your **first message** to Claude Code
after cloning or initialising a repository on any machine. It bootstraps the full
session context: environment detection, constraint activation, agent inventory,
and memory persistence.

**When to use this prompt:**
- First session after `git clone` on any machine
- After a fresh repo creation
- After Claude Code memory has been reset or cleared
- When onboarding a new AI subagent to this repository

---

## ✂️ COPY FROM HERE ─────────────────────────────────────────────────────────

You are starting a new development session in a cross-platform project repository.
Before taking any other action, perform the following steps in order and output the
result of each step before proceeding to the next.

---

### STEP 1 — Read project context files

Read these files in this exact order:

1. `README.md` — understand what this application is and what it does
2. `CLAUDE.md` — your mandatory operating constraints for this repository

Do not proceed to Step 2 until both files have been fully read.
If either file is missing, state which is missing and continue with the other.

---

### STEP 2 — Detect current development environment

Check for the following and report findings:

- Operating system (Windows / macOS / Linux)
- `xcodebuild` in PATH (indicates macOS Apple toolchain present)
- `flutter` and `dart` in PATH (indicates Flutter SDK installed)
- `node` and `npm` in PATH (indicates Node.js / React target available)
- `rustc` and `cargo` in PATH (indicates Rust toolchain installed)
- Any CI environment variable set (e.g. `CI`, `GITHUB_ACTIONS`, `CIRCLECI`)

Output this report exactly:

```
ENVIRONMENT REPORT
─────────────────────────────────────────────────────────
OS:                [Windows | macOS | Linux]
Apple toolchain:   [available | NOT available]
Flutter SDK:       [version X.Y.Z | NOT available]
Dart SDK:          [version X.Y.Z | NOT available]
Node.js:           [version X.Y.Z | NOT available]
Rust toolchain:    [version X.Y.Z | NOT available]
CI mode:           [yes — $VAR_NAME | no]
─────────────────────────────────────────────────────────
Buildable this session:
  [✓/✗] Android     (requires Flutter)
  [✓/✗] Web         (requires Flutter or Node)
  [✓/✗] Windows     (requires Windows OS + Flutter)
  [✓/✗] Linux       (requires Linux OS + Flutter)
  [✓/✗] macOS app   (requires macOS + xcodebuild + Flutter)
  [✓/✗] iOS         (requires macOS + xcodebuild + Flutter)
  [✓/✗] iPadOS      (requires macOS + xcodebuild + Flutter)

NOT buildable this session:
  [list excluded platforms with reason, or "none — all platforms buildable"]
─────────────────────────────────────────────────────────
```

---

### STEP 3 — Check for pending items

Check whether the following files exist and, if so, read them:

- `PENDING_APPLE_CHANGES.md` — Apple-platform changes required from prior sessions
- `SETUP_NOTES.md` — one-time setup steps required on this machine

If either file contains unresolved entries, summarise them and ask:
*"There are pending items from prior sessions. Address these now or proceed with
new work?"*

If both files are empty or contain no actionable items, state: *"No pending items."*

---

### STEP 4 — Inventory available agents and resources

Check for the following directories and list what is present:

**`.claude/agents/`** — specialised agent definitions available in this repo:
- List each `.md` file found and its agent name/purpose (read the `name:` and
  `description:` frontmatter fields from each file).

**`.claude/resources/`** — development reference documents:
- List each file found with a one-line description of its coverage.

**`.claude/templates/`** — workflow templates:
- List each file found.

Output format:

```
AGENT & RESOURCE INVENTORY
─────────────────────────────────────────────────────────
Agents (.claude/agents/):
  • [agent-name] — [description from frontmatter]

Resources (.claude/resources/):
  • [filename] — [coverage summary]

Templates (.claude/templates/):
  • [filename]
─────────────────────────────────────────────────────────
```

If any directory is missing or empty, note it.

---

### STEP 5 — Confirm active constraints

State the following confirmation block verbatim, filling in the bracketed values:

```
CONSTRAINT CONFIRMATION
─────────────────────────────────────────────────────────
Cross-platform guard convention : [PLATFORM: X] comment guards          ✓
Apple-only session restriction  : [ENFORCED — non-macOS | NOT ACTIVE — macOS session]  ✓
Hardcoded path policy           : no absolute paths in committed files   ✓
Plugin compatibility policy     : pub.dev platform matrix check required ✓
CI runner policy                : no local paths in workflow files       ✓
Subagent constraint inheritance : all subagents inherit CLAUDE.md rules  ✓
─────────────────────────────────────────────────────────
```

---

### STEP 6 — Write session context to memory

Write a Claude memory note containing:

- Application name and purpose (from README.md)
- Full platform target matrix
- Current dev machine OS and which platforms are buildable this session
- Available agents and their roles
- Key conventions: comment guard format, Apple constraint, path policy, plugin check

Title the memory: `[repo-name] Session Context — [OS] — [date]`

This ensures future sessions start with correct context without re-reading all files.

---

### STEP 7 — Ready

State:

*"Session initialised on [OS]. [Apple targets ARE / are NOT] available this session.
Available agents: [comma-separated list]. What would you like to work on?"*

Then wait for instructions. **Do not write any code or modify any files until
explicitly instructed.**

---

## ✂️ COPY TO HERE ─────────────────────────────────────────────────────────

---

## Usage Notes

### Expected output from Claude Code

After sending this prompt, Claude Code should produce in order:

1. Confirmation that README.md and CLAUDE.md have been read (Step 1)
2. The Environment Report (Step 2)
3. Pending items summary or "No pending items" (Step 3)
4. Agent & Resource Inventory (Step 4)
5. Constraint Confirmation (Step 5)
6. Memory write confirmation (Step 6)
7. The ready statement (Step 7)

If Claude Code skips a step, prompt: *"You skipped Step [N]. Please complete it."*

### Recovery when memory is stale or wrong

If Claude Code produces an incorrect Environment Report or constraint confirmation,
correct it explicitly:

```
Your environment report is incorrect. The actual environment is:
  OS: macOS
  Apple toolchain: available
  Flutter: 3.27.1
Please update your memory and re-confirm constraints.
```

### Subagent invocation prefix

When spawning a subagent within Claude Code for a focused task, prepend your
task description with:

```
Before starting your task:
1. Read CLAUDE.md in full.
2. Read .claude/agents/[agent-name].md for your role definition.
3. Read any relevant .claude/resources/*.md files.
4. Current OS: [paste OS from Environment Report].
5. Apply all constraints from CLAUDE.md to your work.
```

### Maintenance checklist for this file

| When | Update |
|---|---|
| New agent added to `.claude/agents/` | No change needed — Step 4 discovers dynamically |
| New resource added to `.claude/resources/` | No change needed — Step 4 discovers dynamically |
| New platform target added | Update Step 2 buildable platform list |
| CLAUDE.md schema version bumped | Update Step 5 if new constraints added |
| Memory structure change desired | Update Step 6 instructions |

---

*Schema version: 1.1 — Aligned with CLAUDE.md v1.0 + agent/resource discovery*
