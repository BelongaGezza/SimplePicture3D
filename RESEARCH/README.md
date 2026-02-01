# RESEARCH — Technology Guidance & Knowledge Base

**Purpose:** Curated, up-to-date guidance for SimplePicture3D technologies. Agents use this folder to extend knowledge beyond their cutoff and record findings.

## For All Agents

**Before starting work:**
1. Review `RESEARCH/` for technologies relevant to your task
2. Check `RESEARCH/GOTCHAS.md` for known debugging pitfalls
3. Each technology file lists **Official Sources** and **Last Checked** — treat as the latest version info

**When debugging:**
- Record gotchas, workarounds, and non-obvious fixes in `RESEARCH/GOTCHAS.md`
- Include date, technology, and brief context
- Update the relevant technology file if you discover version-specific or deprecation info

**Knowledge refresh:** Research files cite official sources with "Last checked" dates. When your knowledge cutoff is older than "Last checked", trust the research file. When in doubt, ask the Researcher agent to verify.

## For the Researcher Agent

- **Populate:** Create and maintain one research file per technology used in the app
- **On request:** Search for deprecated code/features, version changes since knowledge cutoff or the research file's last update
- **Official sources:** When finding official docs or changelogs, add them to the technology file with:
  - Source URL
  - Date last checked
  - Key findings or version info

See `.agents/researcher.md` for full responsibilities.

## Research Files Index

| File | Technologies | Last Updated |
|------|--------------|--------------|
| [AI_DEVELOPMENT_GUIDE.md](AI_DEVELOPMENT_GUIDE.md) | Multi-agent workflows, coordination | — |
| [architecture.md](architecture.md) | SimplePicture3D system design | — |
| [tauri.md](tauri.md) | Tauri framework, IPC | — |
| [rust-crates.md](rust-crates.md) | image, stl_io, obj, tokio, serde, anyhow | — |
| [python-ml.md](python-ml.md) | Python, PyTorch, depth estimation | — |
| [frontend.md](frontend.md) | Svelte/React, TypeScript, TailwindCSS | — |
| [threejs.md](threejs.md) | Three.js, WebGL | — |
| [GOTCHAS.md](GOTCHAS.md) | Debugging gotchas (all agents contribute) | — |
