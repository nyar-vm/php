# Platforms and troubleshooting

## Supported hosts

| Host | Platform package | Binary |
|------|------------------|--------|
| Windows x64 | `@nyar/php-win32-x64` | `php.win32-x64-msvc.node` |
| Linux x64 | `@nyar/php-linux-x64` | `php.linux-x64-gnu.node` |
| macOS x64 | `@nyar/php-darwin-x64` | `php.darwin-x64.node` |
| macOS arm64 | `@nyar/php-darwin-arm64` | `php.darwin-arm64.node` |

## Native addon not found

`php version` lists every path it tried. Common causes:

1. The optional dependency was skipped by your package manager
   (`--no-optional`). Install again without that flag.
2. Your OS/CPU is not in the table above.
3. You are running from a source checkout without a built addon.

Build it locally:

```bash
node scripts/build.mjs napi
```

Then point `PHP_NATIVE` at the produced file, or rerun without it after the script
copies the artifact into the platform package.

## Wrong platform

The `os` and `cpu` fields on each platform package prevent installing a mismatched
binary. If you moved `node_modules` between machines, remove it and install again.

## Reporting parser problems

The parser is owned by Boost `language.php`. `@nyar/php` does not rewrite results.
When a diagnostic looks wrong, reproduce it with `php parse file.php --json` and
include that output in the report.
