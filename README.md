# ai-mergetool

A git merge tool that delegates conflict resolution to an AI agent.

![ai-mergetool](https://github.com/user-attachments/assets/8117aead-3641-40c5-ab68-2727d5e09e8e)

`ai-mergetool` follows the standard [git mergetool](https://git-scm.com/docs/git-mergetool) interface, receiving BASE, LOCAL, REMOTE, and MERGED file paths. It builds a prompt from the three input files and a configurable merge strategy, pipes it to an AI agent CLI (e.g. `claude`), and writes the resolved output to the MERGED file. If the agent fails, it falls back to a traditional merge tool.

## Git integration

Add to your `.gitconfig`:

```ini
[merge]
    tool = ai-mergetool
[mergetool "ai-mergetool"]
    cmd = ai-mergetool "$BASE" "$LOCAL" "$REMOTE" "$MERGED"
```

Then resolve conflicts with:

```bash
git mergetool
```

## Usage

```
ai-mergetool <BASE> <LOCAL> <REMOTE> <MERGED> [-c <CONFIG>]
```

- `BASE` — common ancestor file
- `LOCAL` — current branch version
- `REMOTE` — incoming changes version
- `MERGED` — output file for the resolved merge
- `-c, --config` — path to config file (default: `~/.ai-mergetool-config.json`)

## Configuration

On first run, a default config is generated at `~/.ai-mergetool-config.json`:

```json
{
  "agent_command": "claude",
  "agent_args": [
    "--dangerously-skip-permissions",
    "-"
  ],
  "strategy_file": null,
  "default_strategy": "Try to resolve conflicts in a way that the REMOTE changes are preserved as much as possible.",
  "fallback_merge_tool": null
}
```

| Field | Description |
|---|---|
| `agent_command` | Path or command for the AI agent CLI executable |
| `agent_args` | Arguments passed to the agent |
| `strategy_file` | Optional path to a `.md` file with a custom merge strategy |
| `default_strategy` | Strategy text used when no strategy file is set |
| `fallback_merge_tool` | Optional fallback tool (e.g. `vimdiff`, `meld`, `kdiff3`) |

The config file uses JSON5 for parsing, so comments and trailing commas are allowed.

### Custom merge strategy

Create a markdown file describing your preferred merge strategy:

```markdown
Preserve both LOCAL and REMOTE changes when possible.
Prioritize code correctness, then keep additions from both branches.
```

Then point to it in your config:

```json
{
  "strategy_file": "/home/user/.merge-strategy.md"
}
```

## How it works

1. Reads BASE, LOCAL, and REMOTE files
2. Builds a prompt containing the merge strategy and all three file contents
3. Spawns the configured AI agent, sending the prompt via stdin (The agent is expected to be set-up in the configuration to use headless mode, like in this example `echo "say hello world in spanish" | claude --print -`).
4. Reads the agent's merged output from stdout
5. Displays a colored diff of changes (green for additions, red for deletions)
6. Writes the result to the MERGED file

If the agent fails (non-zero exit, empty output), `ai-mergetool` falls back to:
1. The `fallback_merge_tool` from config, or
2. Git's configured `merge.tool`, or
3. Prompts the user to specify a tool interactively

## Example

The `examples/` directory contains a sample merge scenario:

```bash
cd examples/
./run-example.sh
```

## License

Apache 2.0
