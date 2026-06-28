# benletchford.com

Static personal site built from markdown with a small Rust generator.

## Writing

Add markdown files under:

- `content/writing/` for essays and notes

Each file needs YAML front matter:

```yaml
---
title: "Post title"
date: "2026-06-28"
section: "Software"
description: "One sentence summary for indexes, RSS, and metadata."
tags:
  - rust
---
```

Set `draft: true` to keep a file out of the generated site.

Selected Work is a simple markdown list in `content/projects.md`:

```md
- [project-name](https://github.com/benletchford/project-name) - Short description.
```

## Build

```sh
cargo run -- build
```

The generated site is written to `dist/`. GitHub Actions builds the same output
and deploys it to GitHub Pages on pushes to `master` or `main`.

## Dev Server

```sh
cargo run -- dev
```

The dev server builds into `dist/`, serves `http://127.0.0.1:8000/`, and
rebuilds the site whenever a page is refreshed. Use `--port` to choose another
port:

```sh
cargo run -- dev --port 8080
```
