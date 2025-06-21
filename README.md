# frond
## about
`frond` 🌿 -- TUI LLM chat client with branching and complete context control, built in Rust.
## idea
1. **full context control**
	- you choose what the LLM sees. edit, hide, reorder messages. switch roles (user/assistant), split or merge messages.

2. **flexible, branchable conversations**
	- fork from anywhere. explore side paths without losing the main thread. flow data in and out easily. remix and reuse.

3. **structured thinking space**
	- you see the full picture: folders → dialogues → trees → branches → messages. context for the llm stays tight, but you don’t lose the broader map.

## why?

interacting with LLMs is powerful — to learn, code, brainstorm, write — but most clients limit how you manage the conversation.

usual web apps:
- let you edit only user messages (not agent)
- always auto-branch on edit
- don't show token count
- silently drop old messages
- hard to navigate or search past chats

`frond` gives you:
- full context control (edit anything, reorder, hide, split/merge, switch roles)
- clear token usage
- branching, folders, tags
- full edit history and checkpoints
- proper search (by folder/dialogue/branch/message, filtered by tag etc.)

it’s also fast, and TUI makes it easy to pass folder structures, files, or other context to the LLM.

heavily inspired by zed’s text threads — but zed has no branching or extensive history organization features, and web UIs don’t give you this level of control.

## features

- **core**
	- fully editable context (user and assistant messages)
	   - switch roles, split/merge messages, reorder
	- branching conversations
	   - fork from any message, navigate trees and branches
	- token count per branch
	   - know how much context you’re sending
- **editing & history**
	- ctrl+z / ctrl+shift+z for undo/redo
	- per-message edit history (if not last)
	- full tree history (snapshots / checkpoints)
	- clone, detach, attach branches
	- save trees or branches to new dialogue
	- import or mention other dialogues/trees/branches/messages
- **commands & input**
	- special commands: `/file`, `/tree`, `/prompt`, `/now`, `/fetch`
	- support for images/files (not just text)
	- paste folding (large blocks, syntax-aware)
	- syntax highlighting (markdown and code)
	- copy sections between \`\`\` and \`\`\`
- **organization**
	- saved trees and branches
	   - pins, tags + folder-based organization
	- create a dialogue from folder/tag
	   - shows as multiple orphaned trees
	- trash system (per tree, branch, or full dialogue)
	- my prompts (reusable prompt library)
- **navigation & ui**
	- search by folder, dialogue, branch, message
	   - filter by tag, search contents or titles
	- live updates across sessions
	   - editing one view updates others (sqlite-based)
	- gui tabs & windows (planned)
	   - tui is split-friendly already, gui will support tabbed workflows
- **context visibility**
	- hide messages from llm (for notes or pruning)
	   - keeps them in tree, out of prompt
	- optional links/mentions (reference other trees/branches, not sent to llm)
- **later ideas**
	- RAG (chat with documents)
	- mcps and agentic capabilities (multi-chain or planner support)
