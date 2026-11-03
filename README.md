# Rust learning

One repository for everything I build while learning Rust. Each project lives in
its own top-level folder; they are *not* separate repos.

```
Rust/
├── helloWorld/      # rustc-only hello world (hello.rs)
├── hello_cargo/     # cargo project
├── guessing_game/   # cargo project
└── exercises/       # anything else: day folders, snippets, throwaways
```

## Adding a new project

```sh
cd ~/Documents/Learning/Rust
cargo new my_project          # or: mkdir my_project && cd my_project && cargo init
cd my_project && cargo run
cd .. && git add -A && git commit -m "add my_project"
```

Two rules that keep this one repo:

1. **Never nest a repo.** Do not pass `--vcs git` to `cargo new`/`cargo init` here,
   and never `git init` in a subfolder. Plain `cargo new` is safe: cargo detects the
   parent repo and skips git setup. A `pre-commit` hook (`.githooks/pre-commit`)
   blocks the commit if a nested `.git` appears anyway.
2. **Build output is ignored.** `**/target/` and the compiled `helloWorld/hello`
   binary are in `.gitignore`, so only sources, `Cargo.toml`, `Cargo.lock` and
   `.gitignore` files get committed.

Hooks are wired up via `git config core.hooksPath .githooks`; run that once per clone.
