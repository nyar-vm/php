# php

PHP language services packaged for Node.js.

The parser, lexer and highlighter live in Boost `language.php` / `data.text.php`.
This repository ships the Node-API export layer and the npm packages around it.

## Layout

| Path | Role |
|------|------|
| `packages/php.rs/php-npai` | `php-napi` Node-API addon (Rust) |
| `packages/php.ts/platforms/*` | Prebuilt `.node` addons per host |
| `packages/php.ts/packages/php` | Main package: CLI (`@vmz/commander`) + API |
| `packages/php.ts/packages/php-harness` | Diagnostics and parse-latency tuning |
| `packages/php.ts/packages/php-skills` | Agent guides |
| `packages/php.ts/homepage` | VMZ homepage; user docs at `/d/*` (multilingual) |
| `documentation` | Maintainer handbook (English) |

## Build

```bash
cargo check --workspace
node scripts/build.mjs napi          # build addon, copy into the host platform package
node scripts/build-npm-package-js.mjs
```

## CLI

```bash
php version
php parse file.php --json
php tokens file.php
php check src/*.php
php highlight file.php
php-harness bench file.php -n 200
```

## License

MPL-2.0
