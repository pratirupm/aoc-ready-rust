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
$ bun run stats      # 27 pages, 43,490 words, 0 outside 1,200–4,000
$ bun run build      # 28 pages, 69 links prefixed, 8 tables wrapped
$ bun run links      # all 1,200 internal links resolve
```

## Design and accessibility

A rendered pass at 1440×900 and 390×844, in light and dark, covered the landing, Day 1, the setup page, and the year table. Fixes applied:

- Light accent darkened from `#d34a17` (4.41:1) to `#c14315` (5.15:1), passing AA for links and buttons.
- Scrollable code blocks now use the accent for `:focus-visible`; the dark-mode default was invisible.
- Note and tip asides use the rust accent instead of Starlight's stock indigo/purple.
- Card icon tiles use one accent tint instead of four pastel hues.
- Tables are wrapped after build, scroll horizontally on mobile, and carry a “Scroll sideways” hint plus a right-edge fade. The eleven-years table keeps all five columns reachable.
- Prose renders at 17px and code at 15px, as intended.
- The landing content shares the 46rem measure; the fourth card is a real link to the field guide.
- “Start here” and “Ground floor” start collapsed, so the Field guide header is above the fold on chapter pages.
- Mobile tabs wrap instead of clipping; menu, search, copy, and tab targets are at least 44px.
- A tiny `public/a11y.js` adds `aria-expanded`/`aria-controls` to the mobile drawer toggle and names the Pagefind dialog “Search”.
- A `prefers-reduced-motion` guard is in place.

## Deployment

- Repository: [pratirupm/aoc-ready-rust](https://github.com/pratirupm/aoc-ready-rust)
- Live site: [pratirupm.github.io/aoc-ready-rust](https://pratirupm.github.io/aoc-ready-rust/)
- GitHub Actions builds with Bun and deploys `dist/` on every push to `main`.
- The site uses `site` + `base`; `scripts/prefix-links.ts` rewrites 69 hand-written root-relative links and wraps 8 tables after Astro builds.
- The workflow completed: build 16s, deploy 11s. Home, chapter, setup, field guide, fonts, CSS, JS, and sitemap return 200.

## What was not tested

- **Zed's GUI itself.** The Flatpak launches and its CLI, settings path, sandboxed Cargo, tasks, and rust-analyzer toolchain were verified. Opening a window, clicking tasks, and reading diagnostics were not automated.
- **The full Git workflow.** Git commands were syntax-checked, and the first-commit sequence was reproduced in an isolated scratch repository. No real commit was made in a reader's copy during the audit.
- **Every page and interaction.** Four representative pages were rendered at both widths and themes. The production Pagefind search returned five results for “Dijkstra” and Escape closed it. A click-through of all 27 pages and keyboard-only navigation was not exhaustive.
- **External links, checked once.** All 28 external URLs returned 200 on 2026-11-23. They can change later.
- **A custom domain.** GitHub Pages provides the URL above; no CNAME is configured.

## Zed Flatpak

Tested with Zed Flatpak `1.29.3` (`dev.zed.Zed`, Flathub).

- The CLI is `flatpak run dev.zed.Zed`, not `zed`. A wrapper at `~/.local/bin/zed` was created and `zed --version` works.
- Flatpak settings live at `~/.var/app/dev.zed.Zed/config/zed/settings.json`, not `~/.config/zed/settings.json`.
- The sandbox's default `PATH` is `/app/bin:/usr/bin`, so Cargo is invisible. This override was applied and verified:

```console
$ flatpak override --user --env=PATH="$HOME/.cargo/bin:/app/bin:/usr/bin" dev.zed.Zed
$ flatpak run --command=sh dev.zed.Zed -lc 'cargo --version; rust-analyzer --version'
cargo 1.99.1 (5f94df478 2026-08-27)
rust-analyzer 1.99.1 (58cf8f9 2026-10-13)
```

- Inside the sandbox, `cargo test` reports all 41 tests passing and `cargo clippy -D warnings` is clean, so Zed tasks and rust-analyzer have the tools they need.
- Flatpak Zed ships its own `rust-analyzer`; `rustfmt` and `clippy` come from the host toolchain.

## Reproduce

```console
$ bun install
$ bun run check
$ bun run stats
$ bun run build
$ bun run links
$ cd rust-aoc-lab && cargo test && cargo clippy --all-targets -- -D warnings
```
