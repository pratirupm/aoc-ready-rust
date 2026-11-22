---
title: "Meet Zed"
description: "Open a folder, find the panes, run a task, and read a diagnostic."
---

Zed is a text editor built for reading and writing code. It looks plain at first: a list of files, a large space for typing, and a strip along the bottom. Each part has one job, and knowing the four parts is enough to start.

This page is a tour, not a manual. Open a folder, click around, and run one command. The goal is to know where things live before the first program arrives.

## Open a folder

A folder is the unit Zed works with. When you open a folder, Zed reads every file inside it and shows the tree on the left. That tree is the project.

Move into a folder in your terminal and start Zed there:

```console
$ zed .
```

The dot means "this folder". Zed opens a window with the folder name in the title bar. On macOS you can also use File > Open Folder, and the result is the same.

If you have not created the practice project yet, open any folder with a few text files. You can look around without changing anything, and open `rust-aoc-lab` later.

## The four parts of the window

The **project panel** sits on the left. It lists folders and files. A small triangle beside a folder opens it, and a click on a file opens that file in the editor. The panel is where you find a file when you know its name but not its contents.

The **editor** fills the middle. Each open file gets a tab at the top, and the tab shows a dot when the file has unsaved changes. This is where you type. Line numbers run down the left edge, and the current line is highlighted.

The **terminal panel** sits at the bottom, hidden until you ask for it. It runs the same commands you would type in a separate terminal window. Open it with `terminal panel: toggle` from the command palette. Output from tasks appears here as well.

The **Git panel** shares the left side with the project panel. Git is the tool that records snapshots of your files, and each snapshot is called a commit. The panel lists the files that changed since the last snapshot and lets you read the differences. We cover it at the end of this tour.

## The shortcuts worth learning

Zed has hundreds of commands. Four shortcuts carry most of the work at the start, and they follow the same pattern on every system.

| Action | macOS | Linux and Windows |
| --- | --- | --- |
| Open the command palette | `cmd-shift-p` | `ctrl-shift-p` |
| Find a file by name | `cmd-p` | `ctrl-p` |
| Save the current file | `cmd-s` | `ctrl-s` |
| Open your settings | `cmd-,` | `ctrl-,` |

The command palette is the one to remember. Press the keys, type part of a command name, and press Enter. Every editor command in this book can be reached that way, even when you forget a shortcut.

## Save, and let autosave help

Press `cmd-s` on macOS or `ctrl-s` on Linux and Windows after a change. The dot on the tab disappears, and the file on disk matches what you see.

Your settings also turn on autosave when the window loses focus. Click into another application, and Zed saves the file for you. That is convenient, but it is not a reason to skip the save shortcut. A file that has never been saved has no place on disk yet, and the habit keeps you honest about what changed.

## Run a task

A task is a saved command with a name. This book defines four: `cargo test`, `cargo clippy`, `cargo fmt`, and `cargo run`. They live in `.zed/tasks.json` inside the project.

Open the command palette, type `task: spawn`, and press Enter. A list of task names appears. Choose one, and Zed runs it in the terminal panel from the project root, the folder you opened. `task: rerun` repeats the last task without opening the list. That pair is how you will run tests for the rest of the book.

The task output is the same text you would see in a terminal. Read the first error, not the last, because one mistake often produces a pile of messages.

## Read a red squiggle

Rust checks your file as you type. A red wavy underline marks something the checker believes is wrong: a misspelled name, a missing semicolon, a value of the wrong type. Hover the pointer over the underline and a small box appears with the message. Zed calls these reports diagnostics.

The box has a shape. The first line names the problem. The next line points at the file, line, and column. A note or help line often suggests a fix. Move the pointer away and the box closes; the underline stays until the code changes.

A red dot in the gutter, the narrow column beside the line numbers, marks the same problem. Hover the dot for the same message. A yellow underline is a warning rather than an error: the code runs, but clippy or the compiler sees a better way to write it.

A squiggle is not a crash. It is a report from a program that reads your code, and it updates on the next keystroke.

## Format on save

The settings from the setup page run `rustfmt` whenever you save a Rust file. Indentation, spacing, and line breaks change to match the project style. If the file jumps under your cursor, that is the formatter doing its job.

To format every file in the project at once, run the command by hand:

```console
$ cargo fmt
```

The command rewrites files in place and prints nothing when it succeeds. Run it before a commit when you have edited several files.

## The Git panel at a glance

Open the Git panel from the command palette with `git panel: toggle focus`. The panel lists every file that differs from the last snapshot. A mark beside a name tells you whether the file is modified or new.

Click a changed file and the editor shows the file with removed lines marked in red and added lines in green. Thin colored bars in the gutter show where the changes sit in the full file. You do not need to understand every mark today. The panel answers one question: what changed since the last save point?

Git gets its own page later, right before the first commit. For now, notice the panel and leave it alone.

## Where to go next

Close the window when you are done looking. Zed usually reopens the last folder next time. When you are ready to type commands by hand, continue to [Open a terminal](/ground-floor/open-a-terminal/).
