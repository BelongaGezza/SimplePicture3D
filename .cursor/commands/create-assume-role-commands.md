# Create One-Shot Assume-Role Commands for This Sprint

Use this command so an agent can **read the task assignment, find unassigned roles, and create one Cursor command per available role**. Each generated command is a **one-shot role assumption**: when the user runs it (in this or another chat), the AI receives that command as context and acts as that role for the rest of the conversation.

## What You Must Do

1. **Locate the sprint Task Assignment**
   - If the user @-mentioned a file like `SPRINTS/Sprint_1_7/SPRINT_1_7_Task_Assignment.md`, use it.
   - Otherwise find the latest or relevant sprint folder under `SPRINTS/` and open `SPRINT_X_Y_Task_Assignment.md` (e.g. `SPRINT_1_7_Task_Assignment.md`).

2. **Parse the Role Assignment table**
   - In that document, find the "Role Assignment" section and the markdown table with columns: Role | Persona File | Status | Assigned Agent | Owned Tasks | Notes.
   - For each row where **Status** is `Available` and **Assigned Agent** is `-` or empty, that role is unassigned.
   - **Optional:** If the document has a "Roles required for this sprint" section, use it so the user knows which roles must be staffed; you may list required vs optional roles when confirming the created commands.

3. **Create one command file per unassigned role**
   - For each unassigned role, create a new file under `.cursor/commands/` with a predictable name:
     - Filename: `assume-sprint-X-Y-<role-slug>.md`
     - Sprint: use the sprint number from the task assignment path (e.g. `1_7` for Sprint 1.7).
     - Role slug: lowercase, spaces and punctuation replaced by hyphens (e.g. "Junior Engineer 3D" → `junior-engineer-3d`, "Quality Engineer" → `quality-engineer`).
   - Example: `assume-sprint-1-7-junior-engineer-3d.md`, `assume-sprint-1-7-quality-engineer.md`.

4. **Content of each created command file**
   - The file content is the **one-shot prompt** that will be sent as the user message when someone runs that command. It must:
     - State that the AI is assuming the named role for this sprint (one-shot).
     - Tell the AI to read the persona file (from the table) and the Task Assignment.
     - Tell the AI to **claim the role** in the Task Assignment: set Status to `In Progress` and Assigned Agent to a session identifier (e.g. `Cursor-Agent`).
     - List the role’s Owned Tasks and say that in this chat the AI must act only as that role and work on those tasks.
   - Use this structure (adapt placeholders):

```markdown
# Assume role: [Role Name] — Sprint X.Y (one-shot)

You are **[Role Name]** for this conversation. This is a one-shot role assumption for Sprint X.Y.

1. **Claim the role:** Open `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md`, find your role in the Role Assignment table, set Status to `In Progress` and Assigned Agent to `Cursor-Agent` (or your session ID).
2. **Adopt the persona:** Read and follow the persona at [Persona File]. If the table says "(todo.md)" for persona, use the task breakdown and Quality Engineer responsibilities from the Task Assignment.
3. **Identify in chat:** At the start of each substantive reply, identify yourself as **[Role Name]**.
4. **Your tasks:** [Owned Tasks from table]. Work only as this role; use the Task Breakdown and Required Reading in the Task Assignment.

Do the three steps above now, then continue in this chat as **[Role Name]**.
```

5. **Confirm to the user**
   - List the command files you created and the roles they target.
   - Tell the user: "Run any of these commands in a chat to assume that role one-shot: **Cursor Command Palette** → run the command by name (e.g. 'Assume role: Junior Engineer 3D — Sprint 1.7')."

6. **Do not assign the role in the task assignment yourself**
   - This command only *creates* the assume-role command files. The actual claim happens when the user (or another chat) **runs** one of those commands; the AI that runs it will then edit the Task Assignment to claim the role.

## Summary

- **Input:** Sprint Task Assignment (from @ mention or `SPRINTS/Sprint_X_Y/SPRINT_X_Y_Task_Assignment.md`).
- **Output:** One `.cursor/commands/assume-sprint-X-Y-<role-slug>.md` per unassigned role. Each file is a one-shot prompt to become that role and claim it.
- **One-shot agent:** The "agent" is the chat in which the user runs the generated command; that chat then behaves as the chosen role.
