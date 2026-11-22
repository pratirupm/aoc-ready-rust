---
title: "Open a terminal"
description: "Find the terminal, move between folders, and run your first command."
---

A terminal is a window where you talk to your computer by typing. You type a line, press Enter, and the computer answers with text. No buttons, no menus, no mouse.

That sounds plain, and it is. It is also the most direct way to start a program, read what it prints, and see what went wrong. Every tool in this book is used from here.

By the end of this page you can find your way around folders, run a command, and stop one that will not stop on its own.

## The words in the window

A **terminal** is the application. Inside it runs a **shell**: a program that reads your line and does what it says. When we say "the terminal", we mean the window and the shell together.

A **command** is an instruction typed as a line of text. It has a name, sometimes followed by extra words called arguments. `pwd` is a command with no arguments. `cd projects` is a command named `cd` with one argument.

A **folder**, also called a **directory**, is a container for files and other folders. A **file** is a named piece of data: a text document, a photo, or the source code of a program. A **path** is the address of a file or folder, such as `/home/you/projects`.

The `$` in this book is not part of the command. It marks the line where you type. Your real prompt may look like `you@laptop:~$` or `PS C:\Users\you>`. Type what comes after the `$`, then press Enter.

## Open the window

On macOS, press Command-Space, type `Terminal`, and press Enter. The app is called Terminal and also lives in Applications, inside the Utilities folder.

On Linux, open your applications menu and look for Terminal, GNOME Terminal, Konsole, or xterm. Many systems open one with Ctrl-Alt-T.

On Windows, open the Start menu, type `PowerShell`, and press Enter. PowerShell is Microsoft's terminal and shell. If you installed Git during setup, you also have Git Bash, which accepts the macOS and Linux commands on this page.

## Find your place

The folder you are standing in is the **working directory**. The command `pwd`, short for "print working directory", prints its path:

```console
$ pwd
/home/you
```

Yours will show your own user name, and on macOS the path starts with `/Users/` instead of `/home/`. The path `/home/you` is **absolute**: it starts at the root of the file system, the top of the tree.

The command `ls`, short for "list", prints the names inside the current folder:

```console
$ ls
Desktop  Documents  Downloads
```

Your list will differ. Folders and files look the same here, because `ls` prints names only.

To move, use `cd`, short for "change directory", followed by a path:

```console
$ cd Documents
$ pwd
/home/you/Documents
```

`Documents` is a **relative** path: it is read from the folder you are in. An absolute path works from anywhere. If you get lost, `pwd` tells you where you are, and `cd ~` takes you home. The tilde `~` is a shorthand for your home folder.

## Make a folder and step into it

The command `mkdir`, short for "make directory", creates a folder. Run these lines one at a time:

```console
$ cd ~
$ mkdir book-work
$ cd book-work
$ mkdir notes
$ ls
notes
```

`ls` printed the new folder's name. Step into it, look around, and step back up:

```console
$ cd notes
$ pwd
/home/you/book-work/notes
$ cd ..
$ pwd
/home/you/book-work
```

Two dots `..` mean the folder that contains the current one. That is the whole map: `~` is home, `..` is up, a name is down, and a full path works from anywhere.

## Run a command that answers

Now ask the Rust compiler for its version. A **compiler** is a program that turns source code, the text you write, into a program the computer can run.

```console
$ rustc --version
rustc 1.99.1 (58cf8f9c3 2026-10-13)
```

The number will differ. What matters is that a version appears. The argument `--version` asks the command to report its version instead of doing its usual job.

If the answer is `bash: rustc: command not found`, or on PowerShell `The term 'rustc' is not recognized`, the shell looked for a program named `rustc` and did not find one. Either Rust is not installed, or this terminal was opened before the install finished. Close it, open a new one, and try again. If it still fails, see [Set up your workbench](/start/setup/).

## Stop a program

Some commands keep running until you stop them. Press Control-C, written Ctrl-C, to interrupt the program that is running and get your prompt back. The terminal prints `^C` where you pressed it. That is normal, not an error.

## If something went wrong

**`cd: no such file or directory`**: the folder name is not inside the one you are in. Run `ls` to see the names, then check the spelling and capital letters. `Documents` and `documents` are different names.

**`mkdir: cannot create directory ...: File exists`**: the folder is already there. Run `cd` into it instead of making it again.

**`Permission denied`**: you are trying to write outside your home folder. Stay under `~` for this book. Do not put `sudo` in front of these commands; that asks for administrator power you do not need.

**The prompt shows a strange path**: run `cd ~` to return home, then `pwd` to check where you are.

**Nothing happens after Enter**: some commands wait for input or run for a long time. Press Ctrl-C. If there is still no prompt, close the window and open a new terminal.

**PowerShell does not recognize a command**: check the spelling, then look at the table below. Git Bash accepts the commands in the left column.

## Windows differences

| Job | macOS and Linux | PowerShell |
| --- | --- | --- |
| print the current folder | `pwd` | `pwd` or `Get-Location` |
| list names | `ls` | `ls` or `dir` |
| make a folder | `mkdir notes` | `mkdir notes` |
| go up one level | `cd ..` | `cd ..` |
| go home | `cd ~` | `cd ~` |
| find a program | `which rustc` | `Get-Command rustc` |

Git Bash, which comes with Git, follows the left column almost exactly. Paths on Windows use backslashes, so `C:\Users\you` is the same idea as `/home/you`.

## Try it

Make the folder we will use for the rest of the ground floor, and list it:

```console
$ cd ~/book-work
$ mkdir rust-aoc-lab
$ ls
notes  rust-aoc-lab
```

Your list may show more names; the new folder should be among them.

Then step inside and back out once, so your hands learn the way:

```console
$ cd rust-aoc-lab
$ pwd
/home/you/book-work/rust-aoc-lab
$ cd ..
```

Leave the new folder empty for now. Next, we hand the work to Cargo and [run the first program](/ground-floor/your-first-rust-program/).
