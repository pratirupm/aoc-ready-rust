# Verification

What was actually run, on 2026-11-23.

Toolchain: `rustc 1.99.1`, `cargo 1.99.1`, `git 2.56.1`, `bun 1.4.4`.

## The lab

From `rust-aoc-lab/`:

```console
$ cargo fmt --check          # exit 0
$ cargo test                 # 41 tests, all pass
$ cargo test --release       # 41 tests, all pass
$ cargo clippy --all-targets -- -D warnings   # exit 0
```

The 41 tests are 27 library tests plus two per drill, for seven drills.

Each drill was also run directly:

```console
$ cargo run --bin drill01    # 10 / 5
$ cargo run --bin drill02    # 24 / 8
$ cargo run --bin drill03    # 8 / 12
$ cargo run --bin drill04    # 23891448151 / 14016474181
$ cargo run --bin drill05    # 16 / 13
$ cargo run --bin drill06    # 28 / 60
$ cargo run --bin drill07    # 4 / 2
```

## The prose code

The book contains 143 Rust fences. They were audited page by page:

| Section | Fences | Result |
| --- | ---: | --- |
| Ground floor | all | Every runnable block compiled and ran; printed output matched. One compiler line reference was corrected. |
| Days 1–3 | 28 | 27 compile; one is a deliberate signature sketch with no bodies. All titled excerpts match the lab. The Day 2 diagram was wrong and was fixed to match `data/examples/02.txt`. |
| Days 4–7 | 29 | All compile; 28 ran. One fetch/decode sketch is compile-only and does not claim output. Two stale `ways[start]` excerpts were fixed. |
| Start + field guide | all | 4 JSON blocks parse; the `aoc2026` setup was created and tested; all 16 cheat-sheet blocks compiled and ran; all 14 error-decoder “before” programs reproduced their message and all 14 “after” programs compiled. |

`bun run snippets` checks the parts that can drift mechanically: **31 titled lab excerpts** must appear in the lab source in order, and **all 7 example inputs** must appear byte-identically in a book code block.

## The site

```console
$ bun run check      # Astro 0 errors; snippets 0 problems; prose 0 errors, 0 warnings
$ bun run stats      # 27 pages, 43,111 words, 0 outside 1,200–4,000
$ bun run build      # 28 pages
$ bun run links      # all 1,143 internal links resolve
```

The build prints one warning: the sitemap integration is skipped because no `site` URL is configured.

## What was not tested

- **Zed itself.** `zed` is not installed in this environment. The install tabs, command names, JSON settings, tasks, and keybindings were checked against the Zed documentation; `zed .` was not run.
- **The full Git workflow.** Git commands were syntax-checked, and the first-commit sequence was reproduced in an isolated scratch repository. No real commit was made in a reader's copy during the audit.
- **The rendered site.** The HTML build and every internal link were checked, but the browser cache was removed during the audit, so no screenshot pass was taken.
- **Deployment.** No host is configured; `dist/` is the deliverable.

## Reproduce

```console
$ bun install
$ bun run check
$ bun run stats
$ bun run build
$ bun run links
$ cd rust-aoc-lab && cargo test && cargo clippy --all-targets -- -D warnings
```
