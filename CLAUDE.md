# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Requirements

This is a command line tool (ai-mergetool) that has the same command line input interface as a mergetool (like kdfiff3, meld, etc) receiving as arguments the paths to the files involved in a git conflict resolution process (https://git-scm.com/docs/git-mergetool):

> the configured command line will be invoked with BASE set to the name of a temporary file containing the common base for the merge, if available; LOCAL set to the name of a temporary file containing the contents of the file on the current branch; REMOTE set to the name of a temporary file containing the contents of the file to be merged, and MERGED set to the name of the file to which the merge tool should write the result of the merge resolution.

### Concrete functional requirements

- This tool must delegate conflict resolution decision to an AI agent, installed and present in the system where it is being used.

- The configuration file contains the path the the agent CLI interface executable.

- The agent is fed with merge strategy description in `.md` format, the path to the configuration file is part of the tool configuration.

- If the tool is bootstrapped and there is no configuration file (~/.ai-mergetool-config.json), the application must generate one with a default.

- The default statrategy is:

> Try to resolve conflicts in a way that the REMOTE changes are preserved as much as possible.

- It can have a fallback to traditional merge tooling. The fallback merge tool is provided through the configuration file (if present). If not present, it will use github configuration. If none of these are set, it will ask the user.   

- The log output should have descriptions of the applied changes, using colors (green for additions, red for deletes).

## Stack

- Rust
- Prefered self-contained libraries, not bnary linking
- json5 configuration files
- Avoid exoteric libraries, use proven/mainstream libraries when needed. 

## Architecture

This is a cli tool, no UI, no network interfaces.

It must be a self contained binary except for its configuration. If its configuration file is missing, it must generate a default choosing `claude` as default agent tool. 

It intects with the agent through piped stdin, stdout and stderr.

It provides detailed logs in stdout and stdrr.

