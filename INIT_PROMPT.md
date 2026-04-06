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

You are starting a new development session in the SimplePicture3D repository.
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
- `rustc` and `cargo` in PATH (Rust toolchain)
- `node` and `npm` in PATH (Node.js / frontend toolchain)
- `python` or `python3` in PATH and its version (AI backend)
- `xcodebuild` in PATH (macOS — required for signed macOS builds in Phase 3)
- Any CI environment variable set (e.g. `CI`, `GITHUB_ACTIONS`, `CIRCLECI`)

Output this report exactly:

```
ENVIRONMENT REPORT
─────────────────────────────────────────────────────────
OS:                [Windows | macOS | Linux]
Rust toolchain:    [rustc X.Y.Z | NOT available]
Node.js:           [version X.Y.Z | NOT available]
Python:            [version X.Y.Z | NOT available]
Xcode tools:       [available (macOS Phase 3 builds OK) | NOT available]
CI mode:           [yes — $VAR_NAME | no]
─────────────────────────────────────────────────────────
Buildable this session:
  [✓/✗] Windows build    (requires Windows OS + Rust + Node)
  [✓/✗] macOS build      (requires macOS + Rust + Node + xcodebuild for signing)
  [✓/✗] Linux build      (requires Linux OS + Rust + Node + GTK3/WebKit libs)
  [✓/✗] Python AI backend (requires Python 3.10+ + venv)

NOT buildable this session:
  [list excluded targets with reason, or "none — all targets buildable"]
─────────────────────────────────────────────────────────
```

---

### STEP 3 — Check for pending items

Check whether the following files exist and, if so, read them:

- `PENDING_APPLE_CHANGES.md` — macOS/Linux build changes required from prior sessions
- `SETUP_NOTES.md` — one-time setup steps required on this machine

If either file contains unresolved entries, summarise them and ask:
*"There are pending items from prior sessions. Address these now or proceed with
new work?"*

If both files are empty or contain no actionable items, state: *"No pending items."*

---

### STEP 4 — Inventory available agents and resources

Check for the following directories and list what is present:

**`.agents/`** — specialised agent personas for this project:
- List each `.md` file found and its agent name/purpose (read the first heading
  and a brief description from each file).

**`RESEARCH/`** — development reference documents:
- List each file found with a one-line description of its coverage.

**`SPRINTS/`** — sprint tasking templates and artefacts:
- List subdirectories and any top-level template files found.

Output format:

```
AGENT & RESOURCE INVENTORY
─────────────────────────────────────────────────────────
Agents (.agents/):
  • [agent-name] — [role and focus area]

Research (RESEARCH/):
  • [filename] — [coverage summary]

Sprints (SPRINTS/):
  • [subdirectory or file]
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
macOS/Linux Phase 3 restriction : [ENFORCED — Windows session | NOT ACTIVE — macOS/Linux session]  ✓
Hardcoded path policy           : no absolute paths in committed files   ✓
No cloud services               : 100% offline processing required       ✓
License policy                  : no GPL/AGPL deps; Depth-Anything-V2 CC-BY-NC-4.0  ✓
Subagent constraint inheritance : all subagents inherit CLAUDE.md rules  ✓
─────────────────────────────────────────────────────────
```

---

### STEP 6 — Write session context to memory

Write a Claude memory note containing:

- Application name and purpose (from README.md)
- Platform target matrix and current phase
- Current dev machine OS and which builds are available this session
- Available agent roles
- Key conventions: comment guard format, path policy, offline constraint, model license

Title the memory: `SimplePicture3D Session Context — [OS] — [date]`

This ensures future sessions start with correct context without re-reading all files.

---

### STEP 7 — Ready

State:

*"Session initialised on [OS]. [macOS/Linux Phase 3 builds ARE / are NOT] available
this session. Available agents: [comma-separated list]. What would you like to work on?"*

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
  OS: Windows
  Rust: 1.82.0
  Node: 20.11.0
  Python: 3.11.4
Please update your memory and re-confirm constraints.
```

### Subagent invocation prefix

When spawning a subagent within Claude Code for a focused task, prepend your
task description with:

```
Before starting your task:
1. Read CLAUDE.md in full.
2. Read .agents/[agent-name].md for your role definition.
3. Read any relevant RESEARCH/*.md files for your domain.
4. Current OS: [paste OS from Environment Report].
5. Apply all constraints from CLAUDE.md to your work.
```

### Maintenance checklist for this file

| When | Update |
|---|---|
| New agent added to `.agents/` | No change needed — Step 4 discovers dynamically |
| New file added to `RESEARCH/` | No change needed — Step 4 discovers dynamically |
| New platform target added | Update Step 2 buildable target list |
| CLAUDE.md cross-platform section updated | Update Step 5 if new constraints added |
| Memory structure change desired | Update Step 6 instructions |

---

*Schema version: 1.1 — SimplePicture3D adaptation (Tauri/Rust/Svelte/Python; desktop-only)*
