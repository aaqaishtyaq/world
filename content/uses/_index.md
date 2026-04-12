+++
title = "Uses"
weight = 101
+++

## Hardware

- **Apple Macbook Pro 16 (2023)**: Work-issued machine. M3 Max with 36GB RAM.
- **Apple Macbook Pro 14 (Nov 2024)**: Personal machine. M4 Pro with 48GB RAM. This is the machine I use most outside work.
- ~~**Keychron K2 Keyboard**~~: Replaced.
- **Apple Magic Keyboard**: My current desk keyboard. Low profile, reliable, and easy to switch between machines.
- **Logitech MX Master**: Daily mouse. Comfortable for long work sessions and still the easiest default recommendation.
- **Apple iPhone 15 Pro**: Back on USB-C and happy about it.
- **Apple Airpods Pro**: Daily listening and calls.
- **Bose QC 35II**: Still excellent for long meetings and travel.
- **Dell 24" 1080p Monitor**: Basic but dependable. I still prefer 24" at the desk.
- **Raspberry Pi 4 8GB**: Runs a few small personal services and infra experiments.

## Software

- **nix**: Most of my machines use nix in some capacity. It keeps the setup predictable.
- **nixOS**: Raspberry Pi and a few small infra boxes run nixOS. Declarative setup still feels worth the effort.
- **tmux**: The first thing I start in a terminal. Most of my working session lives inside tmux.
- **zsh**: Still my default shell.
- **Ghostty**: Current terminal emulator. Fast, clean, and a better fit for how I work now.
- **Helium Browser**: Current browser in regular use.
- **NetNewsWire**: RSS reader of choice.
- **UTM**: I use it for creating VMs, especially for hosting and testing microVM-related setups.
- **Code Font**: [IBM Plex Mono](https://www.ibm.com/plex/) for editor and terminal.

## AI Agents

### Personal setup

- **Codex**: The agent I use most of the time on my personal machine. It is the default for implementation work, debugging, edits across the codebase, and general iteration. Most of the time I use GPT-5.4 there.
- **Claude Code**: I still use it on personal setup, mostly with Claude Sonnet 4.6, but I hit usage limits pretty quickly.
- **OpenCode + GLM / Kimi K2.5**: What I reach for when I am out of limits elsewhere.

### Work setup

- **Claude Code**: One of the main tools in my work setup. The models there are usually Claude Sonnet 4.6 and Claude Opus 4.6.
- **Cursor**: The other tool I use regularly at work, mostly with Composer 2 and Claude Opus 4.6 in Max mode. I create the plan and tasks list in claude code and let cursor execute on the tasks.
- I usually do not have to think about limits on the work setup.

Most of the setup for the software and machines mentioned here now lives in
[nixpkgs](https://github.com/aaqaishtyaq/nixpkgs). Older history still exists in
[dotfiles](https://github.com/aaqaishtyaq/dotfiles) and
[home_ops](https://github.com/aaqaishtyaq/home_ops), but `nixpkgs` is the current
source of truth.
