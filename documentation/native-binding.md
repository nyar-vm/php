# Native binding

## Crate

`packages/php.rs/php-npai` builds the `php-napi` crate (directory name kept from the
earlier placeholder). It is a `cdylib` for Node and an `rlib` for tests.

Dependencies come from Boost by pinned revision:

```toml
language-php   = { git = "https://github.com/oovm/boost.rs", rev = "<sha>" }
data-text-core = { git = "https://github.com/oovm/boost.rs", rev = "<sha>" }
data-text-php  = { git = "https://github.com/oovm/boost.rs", rev = "<sha>" }
```

All three use the same revision so Cargo resolves a single copy of each crate.
Raising the revision is a deliberate change: run the smoke tests before committing.

## Exports

| Rust | JS | Returns |
|------|----|---------|
| `php_version` | `phpVersion()` | crate version |
| `php_parse` | `phpParse(source)` | `{ ok, error, diagnostics, root, tokenCount }` |
| `php_tokens` | `phpTokens(source)` | `TokenSpan[]` |
| `php_check` | `phpCheck(source)` | `string[]` |
| `php_highlight` | `phpHighlight(source)` | `HighlightSpan[]` |

N-API converts field names to camelCase on the JS side.

## `.node` names

| Host | Target triple | Platform package | File |
|------|---------------|------------------|------|
| Windows x64 | `x86_64-pc-windows-msvc` | `php-win32-x64` | `php.win32-x64-msvc.node` |
| Linux x64 | `x86_64-unknown-linux-gnu` | `php-linux-x64` | `php.linux-x64-gnu.node` |
| macOS x64 | `x86_64-apple-darwin` | `php-darwin-x64` | `php.darwin-x64.node` |
| macOS arm64 | `aarch64-apple-darwin` | `php-darwin-arm64` | `php.darwin-arm64.node` |

`scripts/build.mjs` maps these automatically and copies the artifact into the
matching `platforms/php-<short>/lib/` directory.

## Adding an export

1. Add the pure function to Boost if it does not exist.
2. Wrap it in `php-napi` with `#[napi]` and an English doc comment. `deny(missing_docs)`
   is enabled.
3. Re-export it from `packages/php.ts/packages/php/src/native.ts` and `index.ts`.
4. Add a CLI command or flag only if users need it.
5. Rebuild and run the smoke tests:

```bash
node scripts/build.mjs napi
node packages/php.ts/packages/php/bin/php.mjs parse path/to/file.php
```

`PHP_NATIVE=/absolute/path/to/php.node` lets you validate an addon without
rebuilding platform packages.
