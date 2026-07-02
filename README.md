# benletchford.com

Static personal site built from Markdown with Trunk and a small Rust renderer.

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
cargo install --locked trunk --version 0.21.14
trunk build --release
```

The generated site is written to `dist/`. Trunk copies static assets, runs the
Rust renderer as a build hook, and GitHub Actions deploys the same output to
GitHub Pages on pushes to `master` or `main`.

## Dev Server

```sh
trunk serve
```

The dev server builds into `dist/`, serves `http://127.0.0.1:8000/`, and
rebuilds the site when watched files change. Use `--port` to choose another
port:

```sh
trunk serve --port 8080
```
