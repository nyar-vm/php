# Maintainer handbook

This handbook is for people changing the repository. End-user documentation lives
in the homepage at `/d/*` and is translated; this handbook is English-only.

## What this repository owns

The PHP language implementation is not here. Parsing, tokens and highlighting come
from Boost `language.php` / `data.text.php`. This repository owns:

- the Node-API addon (`php-napi`) that exposes those Rust capabilities to Node.js;
- the prebuilt `.node` platform packages;
- the npm packages (`@nyar/php`, `@nyar/php-harness`, `@nyar/php-skills`);
- the VMZ homepage and its multilingual user docs.

## Layout

| Path | Role |
|------|------|
| `packages/php.rs/php-npai` | `php-napi` Node-API addon (Rust) |
| `packages/php.ts/platforms/*` | Prebuilt `.node` addons per host |
| `packages/php.ts/packages/php` | Main package: CLI + API |
| `packages/php.ts/packages/php-harness` | Diagnostics and tuning harness |
| `packages/php.ts/packages/php-skills` | Agent guides |
| `packages/php.ts/homepage` | VMZ homepage and `/d/*` user docs |
| `documentation` | This maintainer handbook |
| `scripts` | Native build, JS emit and publish preparation |

## Quick start

```bash
pnpm install
cargo check --workspace
pnpm build                 # native addon + JS entries
pnpm --filter @nyar/php-homepage run build
```

Smoke test:

```bash
node packages/php.ts/packages/php/bin/php.mjs version
node packages/php.ts/packages/php/bin/php.mjs parse path/to/file.php
```

## Rules of thumb

1. Keep the Rust crate a bridge. Do not implement parsing or highlighting in
   JavaScript or in `php-napi`.
2. Bump the Boost revision in `Cargo.toml` when you need new language behaviour.
3. Translate user-facing docs; keep code, comments, CI and maintainer docs English.
4. Add a platform package only with a matching build target and `.node` name.

Continue with [Architecture](./architecture.md).
