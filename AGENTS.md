# AGENTS.md

## Project

A terminal-based maze game written in Rust. The maze is procedurally generated using a recursive backtracker algorithm and rendered in the terminal using `crossterm`. The player navigates from a start position to the exit using arrow keys.

## Goal

The goal of this project is to **learn Rust by writing it manually**. This is a deliberate, hands-on exercise — not a production tool. The value is in the process: fighting the borrow checker, understanding ownership, and building something from scratch without shortcuts.

## Human-written code

**All code in this repository is intentionally written by a human.** This is a conscious choice made in the era of AI-generated code. The messiness, the imperfect naming, the small inefficiencies — they are part of the learning process and should be respected.

## AI agent guidelines

### Allowed

- Run the app and report bugs or unexpected behavior
- Give **general, high-level recommendations** (e.g. "this function might panic on edge case X")
- Suggest ideas for new features — briefly, without implementing them
- Maintain and update `README.md` and other documentation files
- Apply **small, precisely scoped changes** that are explicitly described by the human (e.g. "rename this variable", "fix this typo", "add a missing `?` here")
- Minor formatting or style fixes when asked

### Not allowed

- Suggesting or implementing new algorithms
- Rewriting or refactoring whole modules, functions, or the overall app structure
- Writing long pieces of code unprompted
- Replacing human-written logic with "better" alternatives unless explicitly asked
- Making assumptions about what the human "probably wants" and acting on them in code
