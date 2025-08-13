### closest goals
- [ ] action stack mode (undo/redo visualization)
- [ ] implement hiding messages
- [ ] add text search in normal mode (current branch, whole dialogue)
- [ ] add Normal "submodes": Normal > Message/Branch/Tree/Dialogue for additioal actions, fork branch, summarize branch, create tree, rename things etc.
- [ ] copy as text actions: c - copy mode, then copy whole message or copy code block (and implement selection of code blocks to copy)
- [ ] help mode
- [ ] command palette mode
- [ ] save and restore the data to/from disk
- [ ] message selections and operations on them
- [ ] llm apis implementation
- [ ] dialogue overview mode (tree representation of current dialogue's trees, branches + descriptions, maybe tags)
- [ ] org mode (choose dialogues in history, organize in folders, by search)

### long-term thoughts
- [ ] restore focus (tree, branch, dialogue) when user re-opens the dialogue

### priority
Critical for MVP (do these first):

1. Save/restore data to/from disk - Without persistence, users can't actually keep their conversations. This is essential.
2. Help mode - Your interface is complex and powerful, users need discoverability. This validates the UX.
3. Message hiding functionality - This is a key differentiator mentioned in your README ("hide messages from llm"). Core to your value prop.
4. Command palette - Essential for feature discovery and navigation in a TUI with this many actions.
5. Basic LLM API implementation - Just enough to send a branch to an LLM and get a response. This validates the core workflow and makes it actually useful.

Can wait for post-MVP:

- Action stack visualization (nice but not essential)
- Text search (useful but not blocking)
- Advanced copy modes (polish)
- Submodes and advanced organization features
